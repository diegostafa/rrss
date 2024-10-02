use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use super::view::View;
use crate::feed_manager::FeedManager;
use crate::model::filter::Filter;
use crate::model::models::Item;
use crate::tui::app::AppRequest;
use crate::tui::widgets::multiline_paragraph::MultilineParagraph;
use crate::tui::widgets::UiObject;

pub struct DetailedItemView<'a> {
    items: Vec<Item>,
    item_idx: usize,
    header: MultilineParagraph<'a>,
    content: MultilineParagraph<'a>,
}
impl DetailedItemView<'_> {
    pub fn new(items: Vec<Item>, curr_idx: usize) -> Self {
        let mut view = Self {
            items,
            item_idx: curr_idx,
            header: MultilineParagraph::new("".to_string()),
            content: MultilineParagraph::new("".to_string()),
        };
        view.update_view();
        view
    }
    fn item(&self) -> &Item {
        self.items.get(self.item_idx).unwrap()
    }
    fn update_view(&mut self) {
        let item = self.item().clone();
        self.header = MultilineParagraph::new(item.title.clone().unwrap_or_default());
        self.content = MultilineParagraph::new(
            item.content
                .or(item.summary)
                .or(item.media.first().and_then(|c| c.description.clone()))
                .unwrap_or_default(),
        );
        self.set_title();
    }
}
impl View for DetailedItemView<'_> {
    fn title(&self) -> String {
        format!("{}", self.item().title.clone().unwrap_or_default())
    }
    fn refresh(&mut self, _fm: &FeedManager) {
        self.update_view();
    }
    fn specific_update(&mut self, ev: &Event) -> AppRequest {
        self.content.handle_event(ev);
        match ev {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('o') => {
                    if let Some(link) = self.item().links.first()
                        && let Err(e) = open::that_detached(&link.href)
                    {
                        return AppRequest::OpenNotificationView(e.to_string());
                    }
                }
                KeyCode::Char('l') => {
                    return AppRequest::OpenLinksView(
                        Filter::default().with_item_id(self.item().id.clone()),
                    )
                }
                KeyCode::Left => {
                    self.item_idx = self.item_idx.saturating_sub(1).max(0);
                    return AppRequest::RefreshView;
                }
                KeyCode::Right => {
                    self.item_idx = (self.item_idx + 1).min(self.items.len() - 1);
                    return AppRequest::RefreshView;
                }
                KeyCode::Char('i') => {
                    return AppRequest::OpenNotificationView(format!(
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
        let header_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };
        let sep_area = Rect {
            x: area.x,
            y: header_area.y + header_area.height,
            width: area.width,
            height: 1,
        };
        let content_area = Rect {
            x: area.x,
            y: sep_area.y + sep_area.height,
            width: area.width,
            height: area.height - sep_area.height - header_area.height,
        };
        let separator = Block::default().borders(Borders::BOTTOM);
        self.header.draw(f, header_area);
        f.render_widget(separator, sep_area);
        self.content.draw(f, content_area);
    }
}
