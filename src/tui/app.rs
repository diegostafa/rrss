use std::io::{self};
use std::ops::Add;
use std::time::Duration;

use crossterm::event::{self};
use ratatui::widgets::TableState;

use super::views::detailed_item::DetailedItemView;
use super::views::feeds::FeedsView;
use super::views::help::HelpView;
use super::views::items::ItemsView;
use super::views::links::LinksView;
use super::views::notification::NotificationView;
use super::views::prompt::PromptView;
use super::views::status::StatusView;
use super::views::tags::TagView;
use super::views::view::{Dock, DockPosition, ViewController};
use super::{try_init_term, try_release_term};
use crate::feed_manager::{FeedManager, TaskStatus};
use crate::model::filter::Filter;
use crate::model::models::{Feed, FeedId, Item, ItemId, Tag};
use crate::model::sorter::Sorter;

#[derive(Clone)]
pub enum AppRequest {
    None,
    Chain(Vec<AppRequest>),
    RefreshView,
    CloseView,
    CloseDock,
    OpenTagView(Filter, Sorter<Tag>),
    OpenFeedView(Filter, Sorter<Feed>),
    OpenItemsView(FeedId, Sorter<Item>),
    OpenDetailedItemView(Filter, Sorter<Item>, usize),
    OpenLinksView(Filter),
    OpenNotificationView(String),
    OpenHelpView,
    OpenInfoFeedView(FeedId),
    OpenInfoItemView(ItemId),
    OpenSearchDock,
    OpenStatusDock(String),
    SubmitPromptValue(String),
    ChangePromptValue(String),
    UpdateFeeds(Filter),
    UpdateFeed(FeedId),
    OpenItem(ItemId),
    MarkItemAsRead(ItemId),
    MarkFeedAsRead(FeedId),
}
impl AppRequest {
    pub fn or_else<T: FnOnce() -> AppRequest>(self, other: T) -> AppRequest {
        if let AppRequest::None = self {
            return other();
        }
        self
    }
}
impl Add for AppRequest {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self.clone(), other.clone()) {
            (AppRequest::Chain(mut reqs1), AppRequest::Chain(mut reqs2)) => {
                reqs1.append(&mut reqs2);
                AppRequest::Chain(reqs1)
            }
            (AppRequest::Chain(mut reqs1), _) => {
                reqs1.push(other);
                AppRequest::Chain(reqs1)
            }
            (_, AppRequest::Chain(mut reqs2)) => {
                reqs2.insert(0, self);
                AppRequest::Chain(reqs2)
            }
            (_, _) => AppRequest::Chain(vec![self, other]),
        }
    }
}

pub struct App {
    fm: FeedManager,
    vc: ViewController,
}
impl App {
    pub fn new(fm: FeedManager) -> Self {
        let vc = ViewController::new();
        Self { fm, vc }
    }
    pub fn init(mut self) -> Self {
        self.handle_request(AppRequest::OpenFeedView(Filter::new(), Sorter::NONE));
        self
    }
    pub fn run(mut self) -> Result<(), Box<io::Error>> {
        let mut term = try_init_term()?;
        self.vc.curr().set_title();
        while !self.vc.curr().should_close() {
            term.draw(|f| self.vc.draw(f, f.area()))?;
            if event::poll(Duration::from_millis(200))? {
                let req = self.vc.update(&event::read()?);
                self.handle_request(req);
            }
            self.poll_tasks();
        }
        try_release_term(term)
    }

    fn handle_request(&mut self, req: AppRequest) {
        match req {
            AppRequest::None => {}
            AppRequest::Chain(reqs) => reqs.into_iter().for_each(|req| self.handle_request(req)),
            AppRequest::RefreshView => self.vc.curr_mut().refresh(&self.fm),
            AppRequest::CloseDock => self.vc.remove_dock(),
            AppRequest::CloseView => {
                self.vc.pop();
                self.handle_request(AppRequest::RefreshView);
            }
            AppRequest::OpenFeedView(filter, sorter) => {
                self.vc.push(Box::new(FeedsView::new(
                    &self.fm,
                    filter,
                    sorter,
                    TableState::new().with_selected(0),
                )));
            }
            AppRequest::OpenItemsView(feed_id, sorter) => {
                self.fm.increment_feed_hits(feed_id.clone());
                self.vc.push(Box::new(ItemsView::new(
                    &self.fm,
                    Filter::new().feed_id(feed_id),
                    sorter,
                    TableState::new().with_selected(0),
                )));
            }
            AppRequest::OpenTagView(filter, sorter) => {
                self.vc.push(Box::new(TagView::new(
                    &self.fm,
                    filter,
                    sorter,
                    TableState::new().with_selected(0),
                )));
            }
            AppRequest::OpenDetailedItemView(filter, sorter, idx) => {
                let items = self.fm.get_items(&filter, &sorter);
                let item = items.get(idx).unwrap();
                self.fm.mark_item_as_read(item.id.clone());
                let view = DetailedItemView::new(items, idx);
                self.vc.push(Box::new(view));
            }
            AppRequest::OpenLinksView(filter) => {
                let view = LinksView::new(self.fm.get_links(&filter, &Sorter::NONE));
                self.vc.push(Box::new(view));
            }
            AppRequest::OpenNotificationView(msg) => {
                self.vc.push(Box::new(NotificationView::new(msg)));
            }
            AppRequest::OpenHelpView => {
                self.vc.push(Box::new(HelpView::new()));
            }
            AppRequest::OpenInfoFeedView(feed_id) => {
                if let Some(f) = self.fm.get_feed(feed_id) {
                    self.handle_request(AppRequest::OpenNotificationView(format!("{:?}", f.conf)));
                }
            }
            AppRequest::OpenInfoItemView(item_id) => {
                if let Some(i) = self.fm.get_item(item_id) {
                    self.handle_request(AppRequest::OpenNotificationView(format!(
                        "id: {}\ntitle: {}\nfiltered: {}\nread: {}",
                        i.id,
                        i.title.clone().unwrap(),
                        i.is_filtered,
                        i.is_read,
                    )));
                }
            }
            AppRequest::UpdateFeeds(filter) => {
                if let TaskStatus::Running = self.fm.poll_update_feeds() {
                    self.handle_request(AppRequest::OpenNotificationView(format!(
                        "An update is already running"
                    )));
                    return;
                }
                self.handle_request(AppRequest::OpenStatusDock(format!("Updating all feeds...")));
                self.fm.update_feeds(&filter);
            }
            AppRequest::UpdateFeed(feed_id) => {
                if let TaskStatus::Running = self.fm.poll_update_feed() {
                    return;
                }
                self.handle_request(AppRequest::OpenStatusDock(format!("Fetching: {}", feed_id)));
                self.fm.update_feed(feed_id);
            }
            AppRequest::MarkItemAsRead(item_id) => {
                self.fm.mark_item_as_read(item_id);
                self.handle_request(AppRequest::RefreshView);
            }
            AppRequest::MarkFeedAsRead(feed_id) => {
                self.fm.mark_feed_as_read(feed_id);
                self.handle_request(AppRequest::RefreshView);
            }
            AppRequest::OpenItem(item_id) => {
                if let Some(item) = self.fm.get_item(item_id.clone()).cloned()
                    && let Some(link) = &item.links.first()
                {
                    self.handle_request(AppRequest::MarkItemAsRead(item_id));
                    if let Err(e) = open::that_detached(&link.href) {
                        self.handle_request(AppRequest::OpenNotificationView(e.to_string()));
                    }
                }
            }
            AppRequest::OpenStatusDock(msg) => self.vc.try_set_dock(Dock {
                position: DockPosition::Bottom,
                size: 1,
                view: Box::new(StatusView::new(msg)),
            }),
            AppRequest::OpenSearchDock => {
                let dock = Dock {
                    position: DockPosition::Bottom,
                    size: 1,
                    view: Box::new(PromptView::new("Search for: ".to_string())),
                };
                self.vc.try_set_dock(dock);
            }
            AppRequest::SubmitPromptValue(value) => {
                let req = self.vc.curr_mut().on_prompt_submit(value);
                self.handle_request(req);
            }
            AppRequest::ChangePromptValue(value) => {
                let req = self.vc.curr_mut().on_prompt_change(value);
                self.handle_request(req);
                self.handle_request(AppRequest::RefreshView);
            }
        }
    }
    fn poll_tasks(&mut self) {
        if let TaskStatus::Done(res) = self.fm.poll_update_feed() {
            self.handle_request(AppRequest::CloseDock);
            self.handle_request(AppRequest::RefreshView);
            if let Err(e) = res {
                self.handle_request(AppRequest::OpenNotificationView(format!("{:?}", e)));
            }
        }
        if let TaskStatus::Done((errs, _)) = self.fm.poll_update_feeds() {
            self.handle_request(AppRequest::CloseDock);
            self.handle_request(AppRequest::RefreshView);
            if !errs.is_empty() {
                self.handle_request(AppRequest::OpenNotificationView(format!("{:?}", errs)));
            }
        }
    }
}
