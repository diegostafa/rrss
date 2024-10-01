use crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use super::UiObject;

pub struct MultilineParagraph<'a> {
    paragraph: Paragraph<'a>,
    scroll_offset: u16,
    area: Option<Rect>,
}
impl MultilineParagraph<'_> {
    pub fn new(content: String) -> Self {
        Self {
            paragraph: Paragraph::new(content).wrap(Wrap::default()),
            scroll_offset: 0,
            area: None,
        }
    }
}

impl UiObject for MultilineParagraph<'_> {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        self.area = Some(area);
        f.render_widget(&self.paragraph, area);
    }

    fn handle_event(&mut self, ev: &Event) {
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll_offset = self.scroll_offset.saturating_add(1);
                    self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1);
                    self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                }
                KeyCode::PageDown => {
                    self.scroll_offset = self.scroll_offset.saturating_add(10);
                    self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                }
                KeyCode::PageUp => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(10);
                    self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                }
                _ => {}
            },
            Event::Mouse(ev) => {
                let pos = Position {
                    x: ev.column,
                    y: ev.row,
                };
                match ev.kind {
                    MouseEventKind::ScrollUp => {
                        if let Some(area) = self.area
                            && area.contains(pos)
                        {
                            self.scroll_offset = self.scroll_offset.saturating_sub(2);
                            self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if let Some(area) = self.area
                            && area.contains(pos)
                        {
                            self.scroll_offset = self.scroll_offset.saturating_add(2);
                            self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
