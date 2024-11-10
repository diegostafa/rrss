use std::sync::{Arc, Mutex};

use async_semaphore::Semaphore;
use async_std::channel::{Receiver, TryRecvError};
use async_std::task::JoinHandle;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use itertools::{Either, Itertools};
use ratatui_helpers::config::parse_toml;

use crate::cache::CachedFeeds;
use crate::config::{PartialSources, Sources};
use crate::globals::{CONFIG, PROJECT_NAME, SOURCES_FILE};
use crate::model::filter::{Filter, FilterTest};
use crate::model::models::{Feed, FeedData, FeedId, Item, ItemId, Link, Tag};
use crate::model::sorter::Sorter;

pub type RequestError = Box<dyn std::error::Error + Send + Sync>;
pub type FetchResult = Result<(FeedId, FeedData), RequestError>;

pub enum TaskStatus<T> {
    None,
    Running,
    Error(String),
    Done(T),
}

pub struct FeedManager {
    feeds: Vec<Feed>,
    save_mutex: Arc<Mutex<()>>,
    update_feeds_ch: Option<Receiver<Vec<FetchResult>>>,
    update_feed_ch: Option<Receiver<FetchResult>>,
}
impl FeedManager {
    pub fn new() -> Self {
        CachedFeeds::init();
        let feeds = CachedFeeds::load().expect("[error] failed to load feeds");
        let sources = parse_toml::<PartialSources, Sources>(PROJECT_NAME, SOURCES_FILE);
        let fm = Self {
            feeds: sources.to_feeds(feeds),
            save_mutex: Arc::new(Mutex::new(())),
            update_feeds_ch: None,
            update_feed_ch: None,
        };
        let _ = fm.save();
        fm
    }

    pub fn clear(&mut self) {
        self.feeds.iter_mut().for_each(Feed::clear_data);
        let _ = self.save();
    }
    pub fn update_feed(
        &mut self,
        id: FeedId,
        finally: impl FnOnce() + Send + 'static,
    ) -> Option<JoinHandle<()>> {
        if let Some(feed) = self.get_feed(id) {
            let url = feed.url();
            let (sx, rx) = async_std::channel::bounded(1);
            self.update_feed_ch = Some(rx);
            return Some(async_std::task::spawn(async move {
                let res = Self::fetch_feed(url).await;
                sx.send(res).await.unwrap();
                finally();
            }));
        }
        None
    }
    pub fn update_feeds(
        &mut self,
        filter: &Filter,
        finally: impl FnOnce() + Send + 'static,
    ) -> JoinHandle<()> {
        let urls = self
            .get_feeds(filter, &Sorter::NONE)
            .iter()
            .filter(|f| !f.conf.manual_update)
            .map(|f| f.url())
            .collect();

        let (sx, rx) = async_std::channel::bounded(1);
        self.update_feeds_ch = Some(rx);
        async_std::task::spawn(async move {
            let res = Self::fetch_feeds(urls).await;
            sx.send(res).await.unwrap();
            finally();
        })
    }
    pub fn poll_update_feeds(
        &mut self,
    ) -> TaskStatus<(Vec<RequestError>, std::thread::JoinHandle<()>)> {
        match &self.update_feeds_ch {
            None => TaskStatus::None,
            Some(rx) => match rx.try_recv() {
                Err(TryRecvError::Empty) => TaskStatus::Running,
                Err(TryRecvError::Closed) => {
                    self.update_feeds_ch = None;
                    TaskStatus::Error("Internal error".into())
                }
                Ok(feeds) => {
                    self.update_feeds_ch = None;
                    let (ok, err) = feeds
                        .into_iter()
                        .partition_map(|r| r.map_or_else(Either::Right, Either::Left));
                    self.merge_new_feeds(ok);
                    TaskStatus::Done((err, self.save()))
                }
            },
        }
    }
    pub fn poll_update_feed(&mut self) -> TaskStatus<()> {
        match &self.update_feed_ch {
            None => TaskStatus::None,
            Some(rx) => match rx.try_recv() {
                Err(TryRecvError::Empty) => TaskStatus::Running,
                Err(TryRecvError::Closed) => {
                    self.update_feed_ch = None;
                    TaskStatus::Error("Internal error".into())
                }
                Ok(Err(e)) => {
                    self.update_feed_ch = None;
                    TaskStatus::Error(e.to_string())
                }
                Ok(Ok((id, data))) => {
                    self.update_feed_ch = None;
                    if let Some(old_feed) = self.get_feed_mut(id) {
                        old_feed.merge_feed(data);
                        let _ = self.save();
                    }
                    TaskStatus::Done(())
                }
            },
        }
    }
    pub fn merge_new_feeds(&mut self, new_feeds: Vec<(FeedId, FeedData)>) {
        for new in new_feeds {
            if let Some(feed) = self.get_feed_mut(new.0) {
                feed.merge_feed(new.1)
            }
        }
    }
    pub fn mark_item_as_read(&mut self, id: ItemId) {
        if let Some(i) = self.get_item_mut(id) {
            i.state.is_read = true;
            let _ = self.save();
        }
    }
    pub fn mark_feed_as_read(&mut self, id: FeedId) {
        self.items_mut(&Filter::new().feed_id(id))
            .iter_mut()
            .for_each(|i| i.state.is_read = true);
        let _ = self.save();
    }
    pub fn increment_feed_hits(&mut self, id: &FeedId) {
        if let Some(feed) = self.get_feed_mut(id.clone()) {
            feed.increment_hits();
            let _ = self.save();
        }
    }
    pub fn get_tags(&self, filter: &Filter, sorter: &Sorter<Tag>) -> Vec<Tag> {
        self.feeds(&Filter::new())
            .iter()
            .flat_map(|f| &f.conf.tags)
            .counts()
            .into_iter()
            .map(|(k, v)| Tag {
                name: k.to_string(),
                count: v,
            })
            .filter(|t| filter.test(t))
            .sorted_by(sorter.0)
            .collect()
    }
    pub fn get_feeds(&self, filter: &Filter, sorter: &Sorter<Feed>) -> Vec<Feed> {
        self.feeds(filter)
            .into_iter()
            .cloned()
            .sorted_unstable_by(sorter.0)
            .collect()
    }
    pub fn get_items(&self, filter: &Filter, sorter: &Sorter<Item>) -> Vec<Item> {
        self.items(filter)
            .into_iter()
            .cloned()
            .sorted_by(sorter.0)
            .collect()
    }
    pub fn get_links(&self, filter: &Filter, sorter: &Sorter<Link>) -> Vec<Link> {
        self.items(filter)
            .into_iter()
            .flat_map(|i| i.data.links.clone())
            .sorted_by(sorter.0)
            .collect()
    }
    pub fn get_item(&self, id: ItemId) -> Option<&Item> {
        self.items(&Filter::new().item_id(id)).first().cloned()
    }
    pub fn get_item_mut(&mut self, id: ItemId) -> Option<&mut Item> {
        self.items_mut(&Filter::new().item_id(id))
            .into_iter()
            .next()
    }
    pub fn get_feed(&self, id: FeedId) -> Option<&Feed> {
        self.feeds(&Filter::new().feed_id(id)).first().cloned()
    }
    pub fn get_feed_mut(&mut self, id: FeedId) -> Option<&mut Feed> {
        self.feeds_mut(&Filter::new().feed_id(id))
            .into_iter()
            .next()
    }

    fn items(&self, filter: &Filter) -> Vec<&Item> {
        self.feeds
            .iter()
            .filter(|f| filter.test(*f))
            .filter_map(Feed::items)
            .flat_map(|items| items.iter().filter(|i| filter.test(*i)))
            .collect()
    }
    fn items_mut(&mut self, filter: &Filter) -> Vec<&mut Item> {
        self.feeds
            .iter_mut()
            .filter(|f| filter.test(*f))
            .filter_map(Feed::items_mut)
            .flat_map(|items| items.iter_mut().filter(|i| filter.test(*i)))
            .collect()
    }
    fn feeds(&self, filter: &Filter) -> Vec<&Feed> {
        self.feeds.iter().filter(|f| filter.test(*f)).collect()
    }
    fn feeds_mut(&mut self, filter: &Filter) -> Vec<&mut Feed> {
        self.feeds.iter_mut().filter(|f| filter.test(*f)).collect()
    }
    fn save(&self) -> std::thread::JoinHandle<()> {
        std::thread::spawn({
            let guard = self.save_mutex.clone();
            let feeds = self.feeds.clone();
            move || {
                let _guard = guard.lock();
                CachedFeeds::save(&feeds).unwrap();
            }
        })
    }

    async fn fetch_feed(url: String) -> FetchResult {
        async_std::task::spawn_blocking(move || Self::_fetch_feed(&url)).await
    }
    async fn fetch_feeds(urls: Vec<String>) -> Vec<FetchResult> {
        let semaphore = Arc::new(Semaphore::new(CONFIG.max_concurrency));
        let futures = FuturesUnordered::new();
        for url in urls {
            let future = async_std::task::spawn({
                let semaphore = semaphore.clone();
                async move {
                    let _guard = semaphore.acquire().await;
                    async_std::task::spawn_blocking(move || Self::_fetch_feed(&url)).await
                }
            });
            futures.push(future);
        }
        futures.collect().await
    }
    fn _fetch_feed(url: &str) -> FetchResult {
        // todo: use async http client
        let data = ureq::get(url).call()?.into_string()?;
        let data = feed_rs::parser::parse(data.as_bytes()).map(|d| FeedData::from(d, url))?;
        Ok((FeedId(url.to_string()), data))
    }
}
