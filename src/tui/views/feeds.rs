use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_view::stateful_table::{IndexedRow, InteractiveTable, StatefulTable};
use ratatui_view::view::View;

use super::new_indexed_table;
use crate::feed_manager::FeedManager;
use crate::model::filter::Filter;
use crate::model::models::{Feed, Item, Tag};
use crate::model::sorter::Sorter;
use crate::tui::app::{AppRequest, ViewKind};

pub struct FeedsView<'row> {
    table: StatefulTable<'row, IndexedRow<Feed>>,
    filter: Filter,
    sorter: Sorter<Feed>,
}
impl<'row> FeedsView<'row> {
    pub fn new(fm: &FeedManager, filter: Filter, sorter: Sorter<Feed>, state: TableState) -> Self {
        let table = new_indexed_table(fm.get_feeds(&filter, &sorter), state);
        FeedsView {
            table,
            filter,
            sorter,
        }
    }
}
impl View for FeedsView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Feeds
    }
    fn title(&self) -> String {
        format!("rrss - feeds")
    }

    fn refresh(&mut self, fm: &FeedManager) {
        *self = Self::new(
            fm,
            self.filter.clone(),
            self.sorter.clone(),
            self.table.state().clone(),
        );
    }

    fn update(&mut self, ev: &Event) -> AppRequest {
        self.table.update(ev);

        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('f') => {
                    return AppRequest::UpdateFeeds(self.filter.clone());
                }
                KeyCode::Char('t') => {
                    return AppRequest::OpenTagView(Filter::new(), Tag::BY_NAME);
                }
                KeyCode::Char('r') => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::UpdateFeed(id.clone());
                    }
                }
                KeyCode::Char('a') => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::MarkFeedAsRead(id.clone());
                    }
                }
                KeyCode::Char('c') => {
                    self.filter = Filter::new();
                    return AppRequest::RefreshView;
                }
                KeyCode::Char('l') => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::OpenLinksView(Filter::new().feed_id(id.clone()));
                    }
                }
                KeyCode::Char('i') => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::OpenInfoFeedView(id.clone());
                    }
                }
                KeyCode::Enter => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::OpenItemsView(id.clone(), Item::BY_POSTED_REV);
                    }
                }
                _ => {}
            },
            Event::Mouse(ev) => {
                let pos = (ev.row, ev.column);
                match ev.kind {
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(row) = self.table.screen_coords_to_row_index(pos)
                            && let Some(idx) = self.table.selected_index()
                            && row == idx
                            && let Some(id) = self.table.selected_value()
                        {
                            return AppRequest::OpenItemsView(id.clone(), Item::BY_POSTED_REV);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.table.draw(f, area);
    }

    fn on_prompt_submit(&mut self, _value: String) -> AppRequest {
        if let Some(id) = self.table.selected_value() {
            return AppRequest::OpenItemsView(id.clone(), Item::BY_POSTED_REV);
        }
        AppRequest::None
    }
    fn on_prompt_change(&mut self, value: String) -> AppRequest {
        self.filter.feed_contains = Some(value);
        AppRequest::RefreshView
    }
}
