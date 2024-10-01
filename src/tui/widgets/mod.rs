use crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::Frame;

pub mod multiline_paragraph;
pub mod stateful_table;

pub trait UiObject {
    fn draw(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, ev: &Event);
}

#[derive(Default)]
pub struct Padding {
    pub t: u16,
    pub r: u16,
    pub b: u16,
    pub l: u16,
}
impl Padding {
    fn add(&mut self, val: u16) {
        self.t += val;
        self.r += val;
        self.b += val;
        self.l += val;
    }
}
