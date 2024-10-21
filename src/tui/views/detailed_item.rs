use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Borders;
use ratatui::Frame;
use ratatui_helpers::view::View;

use crate::feed_manager::FeedManager;
use crate::model::filter::Filter;
use crate::model::models::Item;
use crate::tui::app::{AppRequest, ViewKind};
use crate::tui::theme::StyledWidget;
use crate::tui::widgets::scrollable_paragraph::ScrollableParagraph;
use crate::tui::widgets::UiObject;

pub struct DetailedItemView<'a> {
    items: Vec<Item>,
    item_idx: usize,
    header: ScrollableParagraph<'a>,
    content: ScrollableParagraph<'a>,

    layout: Layout,
}
impl DetailedItemView<'_> {
    pub fn new(items: Vec<Item>, curr_idx: usize) -> Self {
        let mut view = Self {
            items,
            item_idx: curr_idx,
            header: ScrollableParagraph::new("".to_string()),
            content: ScrollableParagraph::new("".to_string()),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ]),
        };
        view.update_view();
        view
    }
    fn item(&self) -> &Item {
        self.items.get(self.item_idx).unwrap()
    }
    fn update_view(&mut self) {
        let item = self.item().clone();
        self.header = ScrollableParagraph::new(item.title.clone().unwrap_or_default());
        self.content = ScrollableParagraph::new(
            item.content
                .or(item.summary)
                .or(item.media.first().and_then(|c| c.description.clone()))
                .unwrap_or_default(),
        );
        self.set_title();
    }
}
impl View for DetailedItemView<'_> {
    type Model = FeedManager;
    type Signal = AppRequest;
    type Kind = ViewKind;

    fn kind(&self) -> Self::Kind {
        ViewKind::DetailedItem
    }

    fn title(&self) -> String {
        format!("{}", self.item().title.clone().unwrap_or_default())
    }
    fn refresh(&mut self, _fm: &FeedManager) {
        self.update_view();
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        self.content.handle_event(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('o') => {
                    if let Some(link) = self.item().links.first()
                        && let Err(e) = open::that_detached(&link.href)
                    {
                        return AppRequest::OpenPopupView(e.to_string());
                    }
                }
                KeyCode::Char('l') => {
                    return AppRequest::OpenLinksView(Filter::new().item_id(self.item().id.clone()))
                }
                KeyCode::Char('K') | KeyCode::Left => {
                    self.item_idx = self.item_idx.saturating_sub(1).max(0);
                    return AppRequest::RefreshView;
                }
                KeyCode::Char('J') | KeyCode::Right => {
                    self.item_idx = (self.item_idx + 1).min(self.items.len() - 1);
                    return AppRequest::RefreshView;
                }
                KeyCode::Char('i') => {
                    return AppRequest::OpenPopupView(format!(
                        "TITLE: {}\nDATE: {}",
                        self.item().title.clone().unwrap_or_default(),
                        self.item().posted.unwrap_or_default(),
                    ))
                }
                _ => {}
            },
            _ => {}
        }
        AppRequest::None
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let layout = self.layout.split(area);
        self.header.draw(f, layout[0]);
        f.render_widget(StyledWidget::block().borders(Borders::BOTTOM), layout[1]);
        self.content.draw(f, layout[2]);
    }
}
