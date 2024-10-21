use std::fmt::Display;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_helpers::keymap::{KeyMap, ShortCut};

pub enum AppCommand {
    QuitView,
    Search,
    Help,
}
impl Display for AppCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppCommand::QuitView => write!(f, "quit view"),
            AppCommand::Help => write!(f, "help"),
            AppCommand::Search => write!(f, "search"),
        }
    }
}

pub struct AppKeyMap(pub Vec<ShortCut<AppCommand>>);
impl KeyMap for AppKeyMap {
    type Command = AppCommand;
    fn get_shortcuts(&self) -> &[ShortCut<Self::Command>] {
        &self.0
    }

    fn default() -> Self {
        Self(vec![
            ShortCut(
                AppCommand::QuitView,
                vec![
                    KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Left, KeyModifiers::ALT),
                    KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
                ],
            ),
            ShortCut(
                AppCommand::Help,
                vec![
                    KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
                ],
            ),
            ShortCut(
                AppCommand::Search,
                vec![KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)],
            ),
        ])
    }
}
