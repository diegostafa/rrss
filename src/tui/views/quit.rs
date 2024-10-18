use ratatui_view::view::View;

use crate::feed_manager::FeedManager;
use crate::tui::app::AppRequest;

pub struct QuitView;
impl View for QuitView {
    type Model = FeedManager;
    type Signal = AppRequest;

    fn should_close(&self) -> bool {
        true
    }
    fn set_title(&self) {}
}
