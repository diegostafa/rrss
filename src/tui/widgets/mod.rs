use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::Frame;

pub mod multiline_paragraph;

pub trait UiObject {
    fn draw(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, ev: &Event);
}
