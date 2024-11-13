use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_helpers::stateful_table::{IndexedRow, StatefulTable};
use ratatui_helpers::view::View;
use rrss_core::feed_manager::FeedManager;
use rrss_core::filter::Filter;
use rrss_core::models::Tag;
use rrss_core::sorter::Sorter;

use super::centered_rect;
use crate::app::{AppRequest, ViewKind};
use crate::theme::StyledWidget;

pub struct TagView<'row> {
    table: StatefulTable<'row, IndexedRow<Tag>>,
    filter: Filter,
    sorter: Sorter<Tag>,
}
impl TagView<'_> {
    pub fn new(
        fm: &FeedManager,
        filter: Filter,
        sorter: Sorter<Tag>,
        mut state: TableState,
    ) -> Self {
        if state.selected().is_none() {
            state.select(Some(0));
        }
        let table = StyledWidget::indexed_table(fm.get_tags(&filter, &sorter), state, None);
        Self {
            table,
            filter,
            sorter,
        }
    }
}
impl View for TagView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;
    fn kind(&self) -> Self::Kind {
        ViewKind::Tags
    }
    fn title(&self) -> String {
        format!("rrss - tags")
    }
    fn refresh(&mut self, fm: &FeedManager) {
        *self = Self::new(
            fm,
            self.filter.clone(),
            self.sorter.clone(),
            self.table.state().clone(),
        );
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        self.table.update(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('r') => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::UpdateFeeds(Filter::new().tag_id(id.to_string()));
                    }
                }
                KeyCode::Enter => {
                    if let Some(id) = self.table.selected_value() {
                        return AppRequest::CloseView
                            + AppRequest::OpenFeedView(
                                Filter::new().tag_id(id.clone()),
                                Sorter::NONE,
                            );
                    }
                }
                _ => {}
            },
            Event::Mouse(ev) => match ev.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    let pos = Position {
                        x: ev.column,
                        y: ev.row,
                    };
                    if let Some(row) = self.table.screen_coords_to_row_index(pos)
                        && let Some(idx) = self.table.selected_row()
                        && row == idx
                        && let Some(id) = self.table.selected_value()
                    {
                        return AppRequest::CloseView
                            + AppRequest::OpenFeedView(
                                Filter::new().tag_id(id.clone()),
                                Sorter::NONE,
                            );
                    }
                }
                _ => {}
            },
            _ => {}
        }
        AppRequest::None
    }
    fn compute_area(&self, area: Rect) -> Rect {
        let (width, height) = self.table.min_area();
        let (width, height) = (width.min(area.width), height.min(area.height));
        centered_rect(area, (width, height.min(20)))
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.table.draw(f, area)
    }
    fn is_floating(&self) -> bool {
        true
    }
    fn on_prompt_submit(&mut self, _value: String) -> AppRequest {
        if let Some(id) = self.table.selected_value() {
            return AppRequest::OpenFeedView(Filter::new().tag_id(id.to_string()), Sorter::NONE);
        }
        AppRequest::None
    }
    fn on_prompt_change(&mut self, value: String) -> AppRequest {
        self.filter.tag_contains = Some(value);
        AppRequest::RefreshView
    }
}
