use crossterm::event::Event;
use itertools::Itertools;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::TableState;
use ratatui::Frame;

use super::view::View;
use crate::tui::app::AppRequest;
use crate::tui::widgets::stateful_table::{IndexedRow, StatefulTable, Tabular};
use crate::tui::widgets::UiObject;

struct Shortcut {
    name: String,
    shortcut: Vec<String>,
}
impl Tabular for Shortcut {
    type Value = String;

    fn value(&self) -> Self::Value {
        self.name.clone()
    }
    fn content(&self) -> Vec<String> {
        vec![self.name.clone(), self.shortcut.iter().join(",")]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![format!("Name"), format!("Shortcut")])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![Constraint::Length, Constraint::Length]
    }
}

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
        self.table.handle_event(ev);
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.table.draw(f, area)
    }
}
