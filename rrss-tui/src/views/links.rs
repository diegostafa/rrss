use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Clear, TableState};
use ratatui::Frame;
use ratatui_helpers::stateful_table::{IndexedRow, StatefulTable};
use ratatui_helpers::view::View;
use rrss_core::feed_manager::FeedManager;
use rrss_core::models::Link;

use crate::app::{AppRequest, ViewKind};
use crate::theme::StyledWidget;

pub struct LinksView<'row> {
    table: StatefulTable<'row, IndexedRow<Link>>,
}
impl LinksView<'_> {
    pub fn new(links: Vec<Link>) -> Self {
        let table = StyledWidget::indexed_table(links, TableState::new().with_selected(0), None);
        Self { table }
    }
}
impl View for LinksView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Links
    }
    fn title(&self) -> String {
        format!("rrss - links")
    }
    fn is_floating(&self) -> bool {
        true
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        self.table.update(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Enter | KeyCode::Char('o') => {
                    if let Some(id) = self.table.selected_value() {
                        let _ = open::that_detached(id);
                        return AppRequest::CloseView;
                    }
                }
                _ => {}
            },
            Event::Mouse(ev) => {
                let pos = Position {
                    x: ev.column,
                    y: ev.row,
                };
                match ev.kind {
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(row) = self.table.screen_coords_to_row_index(pos)
                            && let Some(idx) = self.table.selected_row()
                            && row == idx
                            && let Some(id) = self.table.selected_value()
                        {
                            let _ = open::that_detached(id);
                            return AppRequest::CloseView;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        AppRequest::None
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(Clear, area);
        self.table.draw(f, area)
    }
}
