use std::fmt::Display;

use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_helpers::keymap::{KeyMap, ShortCut};
use ratatui_helpers::stateful_table::{IndexedRow, StatefulTable};
use ratatui_helpers::view::View;
use rrss_core::feed_manager::FeedManager;
use rrss_core::filter::Filter;
use rrss_core::models::{Feed, Item, Tag};
use rrss_core::sorter::Sorter;

use crate::app::{AppRequest, ViewKind};
use crate::theme::StyledWidget;

pub struct FeedsView<'row> {
    table: StatefulTable<'row, IndexedRow<Feed>>,
    filter: Filter,
    sorter: Sorter<Feed>,
    keymap: FeedsKeyMap,
}
impl<'row> FeedsView<'row> {
    pub fn new(
        fm: &FeedManager,
        filter: Filter,
        sorter: Sorter<Feed>,
        mut state: TableState,
    ) -> Self {
        if state.selected().is_none() {
            state.select(Some(0));
        }
        let table = StyledWidget::indexed_table(fm.get_feeds(&filter, &sorter), state, None);
        FeedsView {
            table,
            filter,
            sorter,
            keymap: KeyMap::default(),
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
            Event::Key(ev) => {
                if let Some(cmd) = self.keymap.get_command(ev) {
                    match cmd {
                        FeedsCommand::UpdateFeeds => {
                            return AppRequest::UpdateFeeds(self.filter.clone())
                        }
                        FeedsCommand::UpdateFeed => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::UpdateFeed(id.clone());
                            }
                        }
                        FeedsCommand::ViewTags => {
                            return AppRequest::OpenTagView(Filter::new(), Tag::BY_NAME)
                        }
                        FeedsCommand::ClearFilters => {
                            self.filter = Filter::new();
                            return AppRequest::RefreshView;
                        }
                        FeedsCommand::MarkFeedAsRead => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::MarkFeedAsRead(id.clone());
                            }
                        }
                        FeedsCommand::ViewFeedLinks => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenLinksView(
                                    Filter::new().feed_id(id.clone()),
                                );
                            }
                        }
                        FeedsCommand::ViewFeedInfo => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenInfoFeedView(id.clone());
                            }
                        }
                        FeedsCommand::OpenFeed => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::OpenItemsView(id.clone(), Item::BY_POSTED_REV);
                            }
                        }
                        FeedsCommand::ClearFeed => {
                            if let Some(id) = self.table.selected_value() {
                                return AppRequest::ClearFeed(id.clone());
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
                            && let Some(id) = self.table.selected_value()
                        {
                            return AppRequest::OpenItemsView(id.clone(), Item::BY_POSTED_REV);
                        }
                        if let Some(col) = self.table.screen_coords_to_col_index(pos)
                            && let Some(idx) = self.table.selected_col()
                            && col == idx
                        {
                            // sort table by col
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

#[derive(Debug)]
pub enum FeedsCommand {
    UpdateFeeds,
    UpdateFeed,
    ViewTags,
    ClearFilters,
    MarkFeedAsRead,
    ViewFeedLinks,
    ViewFeedInfo,
    OpenFeed,
    ClearFeed,
}
impl Display for FeedsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct FeedsKeyMap(pub Vec<ShortCut<FeedsCommand>>);
impl KeyMap for FeedsKeyMap {
    type Command = FeedsCommand;
    fn get_shortcuts(&self) -> &[ShortCut<Self::Command>] {
        &self.0
    }
    fn default() -> Self {
        Self(Vec::from([
            ShortCut(
                FeedsCommand::UpdateFeeds,
                vec![KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::UpdateFeed,
                vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::ClearFilters,
                vec![KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::MarkFeedAsRead,
                vec![KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::ViewFeedLinks,
                vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::ViewFeedInfo,
                vec![KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::ViewTags,
                vec![KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::OpenFeed,
                vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
            ),
            ShortCut(
                FeedsCommand::ClearFeed,
                vec![KeyEvent::new(KeyCode::Char('c'), KeyModifiers::SHIFT)],
            ),
        ]))
    }
}
