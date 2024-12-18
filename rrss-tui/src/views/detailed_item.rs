use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui_helpers::view::View;
use rrss_core::feed_manager::FeedManager;
use rrss_core::filter::Filter;
use rrss_core::globals::CONFIG;
use rrss_core::models::Item;

use crate::app::{AppRequest, ViewKind};
use crate::theme::StyledWidget;
use crate::widgets::scrollable_paragraph::ScrollableParagraph;

pub struct DetailedItemView<'a> {
    items: Vec<Item>,
    item_idx: usize,
    title: Paragraph<'a>,
    content: ScrollableParagraph<'a>,
    layout: Layout,
}
impl DetailedItemView<'_> {
    pub fn new(items: Vec<Item>, curr_idx: usize) -> Self {
        let mut view = Self {
            items,
            item_idx: curr_idx,
            title: StyledWidget::header_paragraph("".to_string()),
            content: ScrollableParagraph::new("".to_string()),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(1), Constraint::Fill(1)]),
        };
        view.update_view();
        view
    }
    fn item(&self) -> &Item {
        self.items.get(self.item_idx).unwrap()
    }
    fn update_view(&mut self) {
        let item = self.item().clone();
        self.title = StyledWidget::header_paragraph(format!(
            "({}/{}) - {}",
            self.item_idx + 1,
            self.items.len(),
            item.data.title.clone().unwrap_or_default()
        ));

        let mut lines = vec![];
        if let Some(title) = item.data.title {
            lines.push(Line::from(format!("Title: {}", title)));
        }
        if let Some(date) = item.data.posted {
            lines.push(Line::from(format!(
                "Posted: {}",
                date.format(CONFIG.theme.date_format.as_str())
            )));
        }
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(
            item.data
                .content
                .or(item.data.summary)
                .or(item
                    .data
                    .media
                    .first()
                    .and_then(|c| c.0.description.as_ref())
                    .map(|a| a.content.clone()))
                .unwrap_or_default(),
        ));

        self.content = ScrollableParagraph::new(lines);
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
        format!("{}", self.item().data.title.clone().unwrap_or_default())
    }
    fn refresh(&mut self, _fm: &FeedManager) {
        self.update_view();
    }
    fn update(&mut self, ev: &Event) -> AppRequest {
        self.content.update(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('o') => {
                    if let Some(link) = self.item().data.links.first()
                        && let Err(e) = open::that_detached(&link.0.href)
                    {
                        return AppRequest::OpenPopupView(e.to_string());
                    }
                }
                KeyCode::Char('l') => {
                    return AppRequest::OpenLinksView(
                        Filter::new().item_id(self.item().data.id.clone()),
                    )
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
                        self.item().data.title.clone().unwrap_or_default(),
                        self.item().data.posted.unwrap_or_default(),
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
        f.render_widget(&self.title, layout[0]);
        self.content.draw(f, layout[1]);
    }
}
