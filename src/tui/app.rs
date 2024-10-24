use std::io::{self};
use std::ops::Add;
use std::time::Duration;

use ratatui::crossterm::event::{self, Event};
use ratatui::widgets::TableState;
use ratatui_helpers::keymap::KeyMap;
use ratatui_helpers::view_controller::ViewController;

use super::keymaps::{AppCommand, AppKeyMap};
use super::views::detailed_item::DetailedItemView;
use super::views::feeds::FeedsView;
use super::views::help::HelpView;
use super::views::items::ItemsView;
use super::views::links::LinksView;
use super::views::popup::PopupView;
use super::views::quit::QuitView;
use super::views::tags::TagView;
use super::{try_init_term, try_release_term};
use crate::feed_manager::{FeedManager, TaskStatus};
use crate::model::filter::Filter;
use crate::model::models::{Feed, FeedId, Item, ItemId, Tag};
use crate::model::sorter::Sorter;

#[derive(PartialEq)]
pub enum ViewKind {
    Feeds,
    Items,
    Tags,
    Links,
    DetailedItem,
    Prompt,
    Help,
    Notification,
    Quit,
}

#[derive(Clone, Default)]
pub enum AppRequest {
    #[default]
    None,
    Chain(Vec<AppRequest>),
    RefreshView,
    CloseView,
    OpenTagView(Filter, Sorter<Tag>),
    OpenFeedView(Filter, Sorter<Feed>),
    OpenItemsView(FeedId, Sorter<Item>),
    OpenDetailedItemView(Filter, Sorter<Item>, usize),
    OpenLinksView(Filter),
    OpenPopupView(String),
    OpenHelpView,
    OpenInfoFeedView(FeedId),
    OpenInfoItemView(ItemId),
    OpenSearchDock,
    CloseSearchDock,
    SubmitPromptValue(String),
    ChangePromptValue(String),
    UpdateFeeds(Filter),
    UpdateFeed(FeedId),
    OpenItem(ItemId),
    MarkItemAsRead(ItemId),
    MarkFeedAsRead(FeedId),
}
impl AppRequest {
    fn or_else<T: FnOnce() -> Self>(self, other: T) -> Self {
        if matches!(self, AppRequest::None) {
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
    vc: ViewController<FeedManager, AppRequest, ViewKind>,
    keymap: AppKeyMap,
}
impl App {
    pub fn new(fm: FeedManager) -> Self {
        Self {
            fm,
            vc: ViewController::new(Box::new(QuitView), Duration::from_secs(3)),
            keymap: KeyMap::default(),
        }
    }
    pub fn init(mut self) -> Self {
        self.handle_request(AppRequest::OpenFeedView(Filter::new(), Sorter::NONE));
        self
    }
    pub fn run(mut self) -> Result<(), Box<io::Error>> {
        let mut term = try_init_term()?;
        self.vc.curr().set_title();
        while self.vc.is_running() {
            let _ = term.draw(|f| self.vc.draw(f, f.area()))?;
            if event::poll(Duration::from_millis(200))? {
                let ev = &event::read()?;
                let req = self
                    .common_update(ev)
                    .or_else(|| self.vc.curr_mut().update(ev));
                self.handle_request(req);
            }
            let req = self.poll_tasks();
            self.handle_request(req);
        }
        try_release_term(term)
    }

    fn common_update(&mut self, ev: &Event) -> AppRequest {
        match ev {
            Event::Key(ev) => match self.keymap.get_command(ev) {
                None => return AppRequest::None,
                Some(cmd) => match cmd {
                    AppCommand::QuitView => return AppRequest::CloseView,
                    AppCommand::Help => return AppRequest::OpenHelpView,
                    AppCommand::Search => return AppRequest::OpenSearchDock,
                },
            },
            _ => {}
        };
        AppRequest::None
    }

    fn handle_request(&mut self, req: AppRequest) {
        match req {
            AppRequest::None => {}
            AppRequest::Chain(reqs) => reqs.into_iter().for_each(|req| self.handle_request(req)),
            AppRequest::RefreshView => self.vc.curr_mut().refresh(&self.fm),
            AppRequest::CloseView => {
                self.vc.pop();
                self.handle_request(AppRequest::RefreshView);
            }
            AppRequest::OpenFeedView(filter, sorter) => self.vc.push(Box::new(FeedsView::new(
                &self.fm,
                filter,
                sorter,
                TableState::new().with_selected(0),
            ))),
            AppRequest::OpenItemsView(feed_id, sorter) => {
                self.fm.increment_feed_hits(&feed_id);
                self.vc.push(Box::new(ItemsView::new(
                    &self.fm,
                    Filter::new().feed_id(feed_id),
                    sorter,
                    TableState::new().with_selected(0),
                )));
            }
            AppRequest::OpenTagView(filter, sorter) => self.vc.push(Box::new(TagView::new(
                &self.fm,
                filter,
                sorter,
                TableState::new().with_selected(0),
            ))),
            AppRequest::OpenDetailedItemView(filter, sorter, idx) => {
                let items = self.fm.get_items(&filter, &sorter);
                let item = items.get(idx).unwrap();
                self.fm.mark_item_as_read(item.id.clone());
                let view = DetailedItemView::new(items, idx);
                self.vc.push(Box::new(view));
            }
            AppRequest::OpenLinksView(filter) => self.vc.push(Box::new(LinksView::new(
                self.fm.get_links(&filter, &Sorter::NONE),
            ))),
            AppRequest::OpenPopupView(msg) => self.vc.push(Box::new(PopupView::new(msg))),
            AppRequest::OpenHelpView => self.vc.push(Box::new(HelpView::new())),

            AppRequest::OpenInfoFeedView(feed_id) => {
                if let Some(f) = self.fm.get_feed(feed_id) {
                    self.handle_request(AppRequest::OpenPopupView(format!("{:?}", f.conf)));
                }
            }
            AppRequest::OpenInfoItemView(item_id) => {
                if let Some(i) = self.fm.get_item(item_id) {
                    self.handle_request(AppRequest::OpenPopupView(format!(
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
                    self.vc
                        .show_status("[fetch feeds error] An update is already running".into());
                    return;
                }
                let id = self.vc.show_status_always("Updating all feeds...".into());
                let finally = {
                    let status = self.vc.status().clone();
                    move || status.lock().unwrap().remove(id)
                };
                let _ = self.fm.update_feeds(&filter, finally);
            }
            AppRequest::UpdateFeed(feed_id) => {
                if let TaskStatus::Running = self.fm.poll_update_feed() {
                    return;
                }
                let id = self.vc.show_status_always(format!("Fetching: {}", feed_id));
                let finally = {
                    let status = self.vc.status().clone();
                    move || status.lock().unwrap().remove(id)
                };
                let _ = self.fm.update_feed(feed_id, finally);
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
                        self.handle_request(AppRequest::OpenPopupView(e.to_string()));
                    }
                }
            }
            AppRequest::CloseSearchDock => self.vc.hide_dock(),
            AppRequest::OpenSearchDock => {
                // self.vc.set_dock(Dock {
                //     position: DockPosition::Bottom,
                //     size: 1,
                //     view: Box::new(PromptView::new("Search for: ".to_string())),
                //     is_hidden: false,
                // });
                todo!();
            }

            AppRequest::SubmitPromptValue(value) => {
                let req = self.vc.curr_mut().on_prompt_submit(value);
                self.handle_request(req);
            }
            AppRequest::ChangePromptValue(value) => {
                let req = self.vc.curr_mut().on_prompt_change(value);
                self.handle_request(req + AppRequest::RefreshView);
            }
        }
    }
    fn poll_tasks(&mut self) -> AppRequest {
        let r1 = match self.fm.poll_update_feed() {
            TaskStatus::Error(e) => {
                self.vc.show_status(e);
                AppRequest::None
            }
            TaskStatus::Done(_) => AppRequest::RefreshView,
            _ => AppRequest::None,
        };
        let r2 = match self.fm.poll_update_feeds() {
            TaskStatus::Error(e) => AppRequest::OpenPopupView(format!("{:?}", e)),
            TaskStatus::Done((errs, _)) => {
                if errs.is_empty() {
                    AppRequest::RefreshView
                } else {
                    AppRequest::RefreshView + AppRequest::OpenPopupView(format!("{:?}", errs))
                }
            }
            _ => AppRequest::None,
        };
        r1 + r2
    }
}
