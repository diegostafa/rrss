use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::{Clear, TableState};
use ratatui::Frame;
use ratatui_view::view::View;
use stateful_table::{IndexedRow, InteractiveTable, StatefulTable};

use crate::feed_manager::FeedManager;
use crate::model::models::Link;
use crate::tui::app::AppRequest;
use crate::tui::widgets::handle_table_events;

pub struct LinksView<'row> {
    table: StatefulTable<'row, IndexedRow<Link>>,
}
impl LinksView<'_> {
    pub fn new(links: Vec<Link>) -> Self {
        let table = StatefulTable::new_indexed(links, TableState::new().with_selected(0));
        Self { table }
    }
}
impl View for LinksView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    fn title(&self) -> String {
        format!("rrss - links")
    }
    fn is_floating(&self) -> bool {
        true
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        handle_table_events(&mut self.table, ev);

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
                let pos = (ev.row, ev.column);
                match ev.kind {
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(row) = self.table.screen_coords_to_row_index(pos)
                            && let Some(idx) = self.table.selected_index()
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
