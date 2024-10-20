use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_view::stateful_table::{IndexedRow, StatefulTable};
use ratatui_view::view::View;

use super::new_indexed_table;
use crate::feed_manager::FeedManager;
use crate::model::models::Shortcut;
use crate::tui::app::{AppRequest, ViewKind};

pub struct HelpView<'row> {
    table: StatefulTable<'row, IndexedRow<Shortcut>>,
}
impl HelpView<'_> {
    pub fn new() -> Self {
        Self {
            table: new_indexed_table(
                vec![
                    Shortcut {
                        name: "Quit".to_string(),
                        shortcut: vec!["q".to_string(), "Esc".to_string()],
                    },
                    Shortcut {
                        name: "Select".to_string(),
                        shortcut: vec!["Enter".to_string()],
                    },
                    Shortcut {
                        name: "Update feed".to_string(),
                        shortcut: vec!["r".to_string()],
                    },
                    Shortcut {
                        name: "Update all visible feeds".to_string(),
                        shortcut: vec!["f".to_string()],
                    },
                ],
                TableState::new().with_selected(0),
            ),
        }
    }
}
impl View for HelpView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Help
    }
    fn title(&self) -> String {
        format!("rrss - help")
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        self.table.update(ev);
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.table.draw(f, area)
    }
}
