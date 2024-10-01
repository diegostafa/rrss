use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use ratatui::Frame;

use super::view::View;
use crate::feed_manager::FeedManager;
use crate::globals::CONFIG;
use crate::model::filter::Filter;
use crate::model::models::Item;
use crate::model::sorter::Sorter;
use crate::tui::app::AppRequest;
use crate::tui::widgets::stateful_table::{app_table, IndexedRow, InteractiveTable, StatefulTable};
use crate::tui::widgets::UiObject;

pub struct ItemsView<'row> {
    table: StatefulTable<'row, IndexedRow<Item>>,
    filter: Filter,
    sorter: Sorter<Item>,
}
impl ItemsView<'_> {
    pub fn new(fm: &FeedManager, filter: Filter, sorter: Sorter<Item>, state: TableState) -> Self {
        let mut items = fm.get_items(&filter, &sorter);
        if !CONFIG.dim_filtered_items {
            items.retain(|i| !i.is_filtered);
        }
        ItemsView {
            table: app_table(items, state),
            filter,
            sorter,
        }
    }
}
impl View for ItemsView<'_> {
    fn title(&self) -> String {
        format!("rrss - items")
    }
    fn refresh(&mut self, fm: &FeedManager) {
        *self = Self::new(
            fm,
            self.filter.clone(),
            self.sorter.clone(),
            self.table.state(),
        );
    }
    fn specific_update(&mut self, ev: &Event) -> AppRequest {
        self.table.handle_event(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('o') => {
                    if let Some(id) = self.table.selected_id() {
                        return AppRequest::OpenItem(id.clone());
                    }
                }
                KeyCode::Char('l') => {
                    if let Some(id) = self.table.selected_id() {
                        return AppRequest::OpenLinksView(
                            Filter::default().with_item_id(id.clone()),
                        );
                    }
                }
                KeyCode::Char('i') => {
                    if let Some(id) = self.table.selected_id() {
                        return AppRequest::OpenInfoItemView(id.clone());
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(feed_id) = &self.filter.with_feed_id {
                        return AppRequest::UpdateFeed(feed_id.clone());
                    }
                }
                KeyCode::Char('a') => {
                    if let Some(id) = self.table.selected_id() {
                        return AppRequest::MarkItemAsRead(id.clone());
                    }
                }
                KeyCode::Char('c') => {
                    self.filter =
                        Filter::default().with_feed_id(self.filter.with_feed_id.clone().unwrap());
                    return AppRequest::RefreshView;
                }
                KeyCode::Enter => {
                    if let Some(idx) = self.table.selected_index() {
                        return AppRequest::OpenDetailedItemView(
                            self.filter.clone(),
                            self.sorter.clone(),
                            idx,
                        );
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
                        {
                            return AppRequest::OpenDetailedItemView(
                                self.filter.clone(),
                                self.sorter.clone(),
                                idx,
                            );
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
        self.table.draw(f, area)
    }
    fn on_prompt_submit(&mut self, _value: String) -> AppRequest {
        if let Some(idx) = self.table.selected_index() {
            return AppRequest::OpenDetailedItemView(self.filter.clone(), self.sorter.clone(), idx);
        }
        AppRequest::None
    }
    fn on_prompt_change(&mut self, value: String) -> AppRequest {
        self.filter.item_contains = Some(value);
        AppRequest::RefreshView
    }
}
