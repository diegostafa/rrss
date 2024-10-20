use std::fmt::Display;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_view::keymap::{KeyMap, ShortCut};

#[derive(PartialEq, Eq, Hash)]
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
    fn get_keymap(&self) -> &[ShortCut<AppCommand>] {
        &self.0
    }
}
impl Default for AppKeyMap {
    fn default() -> Self {
        Self(Vec::from([
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
        ]))
    }
}
