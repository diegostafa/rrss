use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratatui_view::view::View;

use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::centered_rect;

pub struct NotificationView<'a> {
    p: Paragraph<'a>,
}
impl NotificationView<'_> {
    pub fn new(msg: String) -> Self {
        Self {
            p: Paragraph::new(msg)
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL)),
        }
    }
}
impl View for NotificationView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Notification
    }
    fn title(&self) -> String {
        format!("rrss - info")
    }
    fn compute_area(&self, area: Rect) -> Rect {
        let (width, height) = (30, 15);
        let (width, height) = (width.min(area.width), height.min(area.height));
        centered_rect(area, (width, height))
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(&self.p, area);
    }
    fn is_floating(&self) -> bool {
        true
    }
}
