use ratatui::crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use ratatui::Frame;
use ratatui_helpers::stateful_table::{IndexedRow, InteractiveTable, StatefulTable};
use ratatui_helpers::view::View;

use crate::feed_manager::FeedManager;
use crate::model::filter::Filter;
use crate::model::models::Tag;
use crate::model::sorter::Sorter;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::centered_rect;
use crate::tui::theme::StyledWidget;

pub struct TagView<'row> {
    table: StatefulTable<'row, IndexedRow<Tag>>,
    filter: Filter,
    sorter: Sorter<Tag>,
}
impl TagView<'_> {
    pub fn new(fm: &FeedManager, filter: Filter, sorter: Sorter<Tag>, state: TableState) -> Self {
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
                            + AppRequest::CloseView
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
                    let pos = (ev.row, ev.column);
                    if let Some(row) = self.table.screen_coords_to_row_index(pos)
                        && let Some(idx) = self.table.selected_index()
                        && row == idx
                        && let Some(id) = self.table.selected_value()
                    {
                        return AppRequest::CloseView
                            + AppRequest::CloseView
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
        let (width, height) = self.table.size();
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
