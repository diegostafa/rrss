use std::fmt::Display;

use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_helpers::keymap::{KeyMap, ShortCut};
use ratatui_helpers::stateful_table::{IndexedRow, StatefulTable};
use ratatui_helpers::view::View;

use crate::feed_manager::FeedManager;
use crate::model::filter::Filter;
use crate::model::models::Item;
use crate::model::sorter::Sorter;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::theme::StyledWidget;

pub struct ItemsView<'row> {
    table: StatefulTable<'row, IndexedRow<Item>>,
    filter: Filter,
    sorter: Sorter<Item>,
    keymap: ItemsKeyMap,
}
impl ItemsView<'_> {
    pub fn new(
        fm: &FeedManager,
        filter: Filter,
        sorter: Sorter<Item>,
        mut state: TableState,
    ) -> Self {
        if state.selected().is_none() {
            state.select(Some(0));
        }
        ItemsView {
            table: StyledWidget::indexed_table(fm.get_items(&filter, &sorter), state, None),
            filter,
            sorter,
            keymap: KeyMap::default(),
        }
    }
}
impl View for ItemsView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Items
    }
    fn title(&self) -> String {
        format!("rrss - items")
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
            Event::Key(ev) => {
                if let Some(cmd) = self.keymap.get_command(ev) {
                    match cmd {
                        ItemsCommand::OpenItem => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenItem(id.clone());
                            }
                        }
                        ItemsCommand::ViewItem => {
                            if let Some(idx) = self.table.selected_row() {
                                return AppRequest::OpenDetailedItemView(
                                    self.filter.clone(),
                                    self.sorter.clone(),
                                    idx,
                                );
                            }
                        }
                        ItemsCommand::UpdateFeed => {
                            if let Some(feed_id) = &self.filter.feed_id {
                                return AppRequest::UpdateFeed(feed_id.clone());
                            }
                        }
                        ItemsCommand::ClearFilters => {
                            self.filter =
                                Filter::new().feed_id(self.filter.feed_id.clone().unwrap());
                            return AppRequest::RefreshView;
                        }
                        ItemsCommand::MarkItemAsRead => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::MarkItemAsRead(id.clone());
                            }
                        }
                        ItemsCommand::ViewItemInfo => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenInfoItemView(id.clone());
                            }
                        }
                        ItemsCommand::ViewItemLinks => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenLinksView(
                                    Filter::new().item_id(id.clone()),
                                );
                            }
                        }
                    }
                }
            }
            Event::Mouse(ev) => {
                let pos = Position {
                    x: ev.column,
                    y: ev.row,
                };
                match ev.kind {
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(row) = self.table.screen_coords_to_row_index(pos)
                            && let Some(idx) = self.table.selected_row()
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
        if let Some(idx) = self.table.selected_row() {
            return AppRequest::OpenDetailedItemView(self.filter.clone(), self.sorter.clone(), idx);
        }
        AppRequest::None
    }
    fn on_prompt_change(&mut self, value: String) -> AppRequest {
        self.filter.item_contains = Some(value);
        AppRequest::RefreshView
    }
}

#[derive(Debug)]
pub enum ItemsCommand {
    OpenItem,
    ViewItem,
    UpdateFeed,
    ClearFilters,
    MarkItemAsRead,
    ViewItemInfo,
    ViewItemLinks,
}
impl Display for ItemsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct ItemsKeyMap(pub Vec<ShortCut<ItemsCommand>>);
impl KeyMap for ItemsKeyMap {
    type Command = ItemsCommand;
    fn get_shortcuts(&self) -> &[ShortCut<Self::Command>] {
        &self.0
    }
    fn default() -> Self {
        Self(Vec::from([
            ShortCut(
                ItemsCommand::OpenItem,
                vec![KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::ViewItem,
                vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::UpdateFeed,
                vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::MarkItemAsRead,
                vec![KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::ViewItemInfo,
                vec![KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::ClearFilters,
                vec![KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)],
            ),
            ShortCut(
                ItemsCommand::ViewItemLinks,
                vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)],
            ),
        ]))
    }
}
