use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui_view::view::View;

use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};

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
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Status
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(self.msg.clone(), area)
    }
}
