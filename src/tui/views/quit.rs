use ratatui_helpers::view::View;

use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};

pub struct QuitView;
impl View for QuitView {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Quit
    }
    fn set_title(&self) {}
}
