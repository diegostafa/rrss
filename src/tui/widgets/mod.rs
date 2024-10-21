use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::Frame;

pub mod scrollable_paragraph;
pub mod tui_input; // copied from the crate tui_input

pub trait UiObject {
    fn draw(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, ev: &Event);
}
