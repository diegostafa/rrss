use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui_view::view::View;
use tui_input::backend::crossterm::EventHandler;
use tui_input::{Input, StateChanged};

use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};

pub struct PromptView {
    prefix: String,
    input: Input,
}

impl PromptView {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            input: Input::default(),
        }
    }
}
impl View for PromptView {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Prompt
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        match ev {
            Event::Key(key) => match key.code {
                KeyCode::Esc => return AppRequest::CloseDock,
                KeyCode::Enter => {
                    return AppRequest::CloseDock
                        + AppRequest::SubmitPromptValue(self.input.value().to_string())
                }
                _ => {}
            },
            _ => {}
        }
        if let Some(StateChanged { value: true, .. }) = self.input.handle_event(todo!()) {
            return AppRequest::ChangePromptValue(self.input.value().to_string());
        }
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let p = Paragraph::new(self.prefix.clone() + self.input.value());
        f.render_widget(p, area)
    }
}
