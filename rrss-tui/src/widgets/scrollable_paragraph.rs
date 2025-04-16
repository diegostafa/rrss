use std::fmt::Display;

use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;
use ratatui_helpers::keymap::{KeyMap, ShortCut};

pub struct ScrollableParagraph<'a> {
    paragraph: Paragraph<'a>,
    scroll: u16,
    area: Rect,
    keymap: ScrollableParagraphKeyMap,
}
impl<'a> ScrollableParagraph<'a> {
    pub fn new(content: impl Into<Text<'a>>) -> Self {
        Self {
            paragraph: Paragraph::new(content).wrap(Wrap::default()),
            scroll: 0,
            area: Rect::default(),
            keymap: KeyMap::default(),
        }
    }
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        self.area = area;
        f.render_widget(&self.paragraph, area);
    }
    pub fn update(&mut self, ev: &Event) {
        match ev {
            Event::Key(ev) => {
                if let Some(cmd) = self.keymap.get_command(ev) {
                    match cmd {
                        ScrollableParagraphCommand::GoDown => {
                            self.scroll = self.scroll.saturating_add(1).min(self.max_scroll());
                            self.scroll_paragraph();
                        }
                        ScrollableParagraphCommand::GoUp => {
                            self.scroll = self.scroll.saturating_sub(1);
                            self.scroll_paragraph();
                        }
                        ScrollableParagraphCommand::GoPageDown => {
                            self.scroll = self
                                .scroll
                                .saturating_add(self.area.height)
                                .min(self.max_scroll());
                            self.scroll_paragraph();
                        }
                        ScrollableParagraphCommand::GoPageUp => {
                            self.scroll = self.scroll.saturating_sub(self.area.height);
                            self.scroll_paragraph();
                        }
                    }
                }
            }

            Event::Mouse(ev) => {
                let pos = Position {
                    x: ev.column,
                    y: ev.row,
                };
                if self.area.contains(pos) {
                    match ev.kind {
                        MouseEventKind::ScrollUp => {
                            self.scroll = self.scroll.saturating_sub(2);
                            self.scroll_paragraph();
                        }
                        MouseEventKind::ScrollDown => {
                            self.scroll = self.scroll.saturating_add(2).min(self.max_scroll());
                            self.scroll_paragraph();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn scroll_paragraph(&mut self) {
        self.paragraph = self.paragraph.clone().scroll((self.scroll, 0));
    }
    fn max_scroll(&self) -> u16 {
        self.paragraph.line_count(self.area.width).saturating_sub(1) as u16
    }
}

#[derive(Debug, Clone)]
pub enum ScrollableParagraphCommand {
    GoDown,
    GoUp,
    GoPageDown,
    GoPageUp,
}
impl Display for ScrollableParagraphCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct ScrollableParagraphKeyMap(pub Vec<ShortCut<ScrollableParagraphCommand>>);
impl KeyMap for ScrollableParagraphKeyMap {
    type Command = ScrollableParagraphCommand;
    fn get_shortcuts(&self) -> &[ShortCut<Self::Command>] {
        &self.0
    }
    fn default() -> Self {
        Self(Vec::from([
            ShortCut(
                ScrollableParagraphCommand::GoDown,
                vec![
                    KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
                ],
            ),
            ShortCut(
                ScrollableParagraphCommand::GoUp,
                vec![
                    KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
                ],
            ),
            ShortCut(
                ScrollableParagraphCommand::GoPageDown,
                vec![KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)],
            ),
            ShortCut(
                ScrollableParagraphCommand::GoPageUp,
                vec![KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)],
            ),
        ]))
    }
}
