use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::views::view::View;

pub struct StatusView<'a> {
    msg: Paragraph<'a>,
}
impl StatusView<'_> {
    pub fn new(msg: String) -> Self {
        Self {
            msg: Paragraph::new(msg),
        }
    }
}
impl View for StatusView<'_> {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(self.msg.clone(), area)
    }
}
