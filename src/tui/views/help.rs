use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_helpers::keymap::{KeyMap, ShortCut};
use ratatui_helpers::stateful_table::StatefulTable;
use ratatui_helpers::view::View;

use super::feeds::{FeedsCommand, FeedsKeyMap};
use super::items::{ItemsCommand, ItemsKeyMap};
use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::keymaps::{AppCommand, AppKeyMap};
use crate::tui::theme::StyledWidget;

pub struct HelpView<'a> {
    app_table: StatefulTable<'a, ShortCut<AppCommand>>,
    feeds_table: StatefulTable<'a, ShortCut<FeedsCommand>>,
    items_table: StatefulTable<'a, ShortCut<ItemsCommand>>,
    layout: Layout,
}
impl HelpView<'_> {
    pub fn new() -> Self {
        Self {
            app_table: StyledWidget::table(
                AppKeyMap::default().0,
                TableState::default(),
                Some("Global Shortcuts".into()),
            ),
            feeds_table: StyledWidget::table(
                FeedsKeyMap::default().0,
                TableState::default(),
                Some("Feeds Shortcuts".into()),
            ),
            items_table: StyledWidget::table(
                ItemsKeyMap::default().0,
                TableState::default(),
                Some("items Shortcuts".into()),
            ),
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ]),
        }
    }
}
impl View for HelpView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> ViewKind {
        ViewKind::Help
    }
    fn update(&mut self, ev: &Event) -> Self::Signal {
        // todo: only update focused tables
        self.app_table.update(ev);
        self.feeds_table.update(ev);
        self.items_table.update(ev);
        Self::Signal::default()
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let layout = self.layout.split(area);
        self.app_table.draw(f, layout[0]);
        self.feeds_table.draw(f, layout[1]);
        self.items_table.draw(f, layout[2]);
    }
}
