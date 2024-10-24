use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui_helpers::view::View;

use crate::feed_manager::FeedManager;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::widgets::tui_input::{to_input_request, Input, StateChanged};

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
                KeyCode::Esc => {
                    return AppRequest::ChangePromptValue("".to_string())
                        + AppRequest::CloseSearchDock
                }
                KeyCode::Enter => {
                    return AppRequest::CloseSearchDock
                        + AppRequest::SubmitPromptValue(self.input.value().to_string())
                }
                _ => {}
            },
            _ => {}
        }
        if let Some(req) = to_input_request(ev) {
            if let Some(StateChanged { value: true, .. }) = self.input.handle(req) {
                return AppRequest::ChangePromptValue(self.input.value().to_string());
            }
        }
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let p = Paragraph::new(self.prefix.clone() + self.input.value());
        f.render_widget(p, area)
    }
}
