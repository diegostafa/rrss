use ratatui_helpers::view::View;
use rrss_core::feed_manager::FeedManager;

use crate::app::{AppRequest, ViewKind};

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
