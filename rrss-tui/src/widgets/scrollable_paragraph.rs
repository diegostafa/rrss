use ratatui::crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

pub struct ScrollableParagraph<'a> {
    paragraph: Paragraph<'a>,
    scroll: u16,
    area: Rect,
}
impl<'a> ScrollableParagraph<'a> {
    pub fn new(content: impl Into<Text<'a>>) -> Self {
        Self {
            paragraph: Paragraph::new(content).wrap(Wrap::default()),
            scroll: 0,
            area: Rect::default(),
        }
    }
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        self.area = area;
        f.render_widget(&self.paragraph, area);
    }
    pub fn update(&mut self, ev: &Event) {
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll = self.scroll.saturating_add(1).min(self.max_scroll());
                    self.scroll_paragraph();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll = self.scroll.saturating_sub(1);
                    self.scroll_paragraph();
                }
                KeyCode::PageDown => {
                    self.scroll = self
                        .scroll
                        .saturating_add(self.area.height)
                        .min(self.max_scroll());
                    self.scroll_paragraph();
                }
                KeyCode::PageUp => {
                    self.scroll = self.scroll.saturating_sub(self.area.height);
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
