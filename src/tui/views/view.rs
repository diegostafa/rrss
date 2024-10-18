use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal;
use ratatui::layout::Rect;
use ratatui::widgets::Clear;
use ratatui::Frame;

use super::quit::QuitView;
use crate::feed_manager::FeedManager;
use crate::tui::app::AppRequest;

pub trait View {
    fn should_close(&self) -> bool {
        false
    }
    fn is_floating(&self) -> bool {
        false
    }
    fn set_title(&self) {
        crossterm::execute!(std::io::stdout(), terminal::SetTitle(self.title())).unwrap()
    }
    fn compute_draw_area(&self, area: Rect) -> Rect {
        area
    }
    fn title(&self) -> String {
        String::new()
    }
    fn refresh(&mut self, _fm: &FeedManager) {}
    fn draw(&mut self, _f: &mut Frame<'_>, _area: Rect) {}
    fn specific_update(&mut self, _ev: &Event) -> AppRequest {
        AppRequest::None
    }
    fn on_prompt_submit(&mut self, _value: String) -> AppRequest {
        AppRequest::None
    }
    fn on_prompt_change(&mut self, _value: String) -> AppRequest {
        AppRequest::None
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Left if ev.modifiers == KeyModifiers::ALT => return AppRequest::CloseView,
                KeyCode::Char('q') => return AppRequest::CloseView,
                KeyCode::Char('?') => return AppRequest::OpenHelpView,
                KeyCode::Char('/') => return AppRequest::OpenSearchDock,
                _ => {}
            },
            _ => {}
        };
        self.specific_update(ev)
    }
}

pub struct ViewController {
    views: Vec<Box<dyn View>>,
    dock: Option<Dock>,
}
impl ViewController {
    pub fn new() -> Self {
        Self {
            views: vec![Box::new(QuitView)],
            dock: None,
        }
    }
    pub fn push(&mut self, view: Box<dyn View>) {
        self.views.push(view);
        self.curr().set_title();
    }
    pub fn pop(&mut self) {
        self.views.pop();
        self.curr().set_title();
    }
    pub fn curr(&self) -> &dyn View {
        self.views.last().unwrap().as_ref()
    }
    pub fn curr_mut(&mut self) -> &mut Box<dyn View> {
        self.views.last_mut().unwrap()
    }
    pub fn prev_mut(&mut self) -> &mut Box<dyn View> {
        let len = self.views.len();
        self.views.get_mut(len - 2).unwrap()
    }
    pub fn all(&mut self) -> &mut Vec<Box<dyn View>> {
        &mut self.views
    }
    pub fn try_set_dock(&mut self, dock: Dock) {
        if self.dock.is_none() {
            self.dock = Some(dock);
        }
    }
    pub fn remove_dock(&mut self) {
        self.dock = None;
    }
    fn draw_bottom_up(&mut self, f: &mut Frame<'_>, area: Rect, idx: usize) {
        if self.views[idx].is_floating() {
            self.draw_bottom_up(f, area, idx - 1);
            let view = &mut self.views[idx];
            let area = view.compute_draw_area(area);
            f.render_widget(Clear, area);
            view.draw(f, area);
        } else {
            self.views[idx].draw(f, area);
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, mut area: Rect) {
        if let Some(dock) = &mut self.dock {
            let (dock_area, other_area) = dock.split_area(area);
            area = other_area;
            dock.view.draw(f, dock_area);
        }
        if self.curr_mut().is_floating() {
            self.draw_bottom_up(f, area, self.views.len() - 1);
        } else {
            self.curr_mut().draw(f, area);
        }
    }
    pub fn update(&mut self, ev: &Event) -> AppRequest {
        if let Some(dock) = &mut self.dock {
            dock.view
                .specific_update(ev)
                .or_else(|| self.curr_mut().update(ev))
        } else {
            self.curr_mut().update(ev)
        }
    }
}

pub type DockSize = u16;
#[derive(PartialEq)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
}
pub struct Dock {
    pub position: DockPosition,
    pub size: DockSize,
    pub view: Box<dyn View>,
}
impl Dock {
    pub fn split_area(&self, area: Rect) -> (Rect, Rect) {
        match self.position {
            DockPosition::Left => (
                Rect {
                    x: area.x,
                    y: area.y,
                    width: self.size,
                    height: area.height,
                },
                Rect {
                    x: area.x + self.size,
                    y: area.y,
                    width: area.width.saturating_sub(self.size),
                    height: area.height,
                },
            ),
            DockPosition::Right => (
                Rect {
                    x: area.x + area.width.saturating_sub(self.size),
                    y: area.y,
                    width: self.size,
                    height: area.height,
                },
                Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width.saturating_sub(self.size),
                    height: area.height,
                },
            ),
            DockPosition::Top => (
                Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: area.y + self.size,
                },
                Rect {
                    x: area.x,
                    y: area.y + self.size,
                    width: area.width,
                    height: area.height.saturating_sub(self.size),
                },
            ),
            DockPosition::Bottom => (
                Rect {
                    x: area.x,
                    y: area.y + area.height.saturating_sub(self.size),
                    width: area.width,
                    height: self.size,
                },
                Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: area.height.saturating_sub(self.size),
                },
            ),
        }
    }
}
