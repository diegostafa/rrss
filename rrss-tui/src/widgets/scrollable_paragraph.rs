use ratatui::crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

pub struct ScrollableParagraph<'a> {
    paragraph: Paragraph<'a>,
    scroll_offset: u16,
    area: Rect,
}
impl<'a> ScrollableParagraph<'a> {
    pub fn new(content: impl Into<Text<'a>>) -> Self {
        Self {
            paragraph: Paragraph::new(content).wrap(Wrap::default()),
            scroll_offset: 0,
            area: Rect::default(),
        }
    }
    pub fn scroll_paragraph(&mut self) {
        self.paragraph = self.paragraph.clone().scroll((self.scroll_offset, 0));
    }
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        self.area = area;
        f.render_widget(&self.paragraph, area);
    }
    pub fn update(&mut self, ev: &Event) {
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
