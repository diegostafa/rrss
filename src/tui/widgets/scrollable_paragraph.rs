use ratatui::crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use super::UiObject;

pub struct ScrollableParagraph<'a> {
    paragraph: Paragraph<'a>,
    scroll_offset: u16,
    area: Rect,
}
impl ScrollableParagraph<'_> {
    pub fn new(content: String) -> Self {
        Self {
            paragraph: Paragraph::new(content).wrap(Wrap::default()),
            scroll_offset: 0,
            area: Rect::default(),
        }
    }
    pub fn scroll_paragraph(&mut self) {
        self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
    }
}

impl UiObject for ScrollableParagraph<'_> {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        self.area = area;
        f.render_widget(&self.paragraph, area);
    }

    fn handle_event(&mut self, ev: &Event) {
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll_offset = self.scroll_offset.saturating_add(1);
                    self.scroll_paragraph();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1);
                    self.scroll_paragraph();
                }
                KeyCode::PageDown => {
                    self.scroll_offset = self.scroll_offset.saturating_add(self.area.height);
                    self.scroll_paragraph();
                }
                KeyCode::PageUp => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(self.area.height);
                    self.scroll_paragraph();
                }
                _ => {}
            },
            Event::Mouse(ev) => {
                let pos = Position {
                    x: ev.column,
                    y: ev.row,
                };
                if !self.area.contains(pos) {
                    return;
                }
                match ev.kind {
                    MouseEventKind::ScrollUp => {
                        self.scroll_offset = self.scroll_offset.saturating_sub(2);
                        self.scroll_paragraph();
                    }
                    MouseEventKind::ScrollDown => {
                        self.scroll_offset = self.scroll_offset.saturating_add(2);
                        self.scroll_paragraph();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
