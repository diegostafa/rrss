use crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use ratatui::Frame;
use stateful_table::{IndexedRow, StatefulTable};

use super::view::View;
use crate::model::models::Shortcut;
use crate::tui::app::AppRequest;
use crate::tui::widgets::handle_table_events;

pub struct HelpView<'row> {
    table: StatefulTable<'row, IndexedRow<Shortcut>>,
}
impl HelpView<'_> {
    pub fn new() -> Self {
        Self {
            table: StatefulTable::new_indexed(
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
    fn title(&self) -> String {
        format!("rrss - help")
    }
    fn specific_update(&mut self, ev: &Event) -> AppRequest {
        handle_table_events(&mut self.table, ev);
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.table.draw(f, area)
    }
}
