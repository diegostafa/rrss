use std::fmt::Display;

use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui_helpers::keymap::{KeyMap, ShortCut};
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
    keymap: DetailedItemKeyMap,
}
impl DetailedItemView<'_> {
    pub fn new(items: Vec<Item>, curr_idx: usize) -> Self {
        let mut view = Self {
            items,
            item_idx: curr_idx,
            title: StyledWidget::header_paragraph("".to_string()),
            content: ScrollableParagraph::new(""),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(1), Constraint::Fill(1)]),
            keymap: KeyMap::default(),
        };
        view.update_view();
        view
    }
    fn item(&self) -> &Item {
        self.items.get(self.item_idx).unwrap()
    }
    fn update_view(&mut self) {
        let mut item = self.item().clone();
        let title = std::mem::take(&mut item.data.title);

        self.title = StyledWidget::header_paragraph(format!(
            "({}/{}) - {}",
            self.item_idx + 1,
            self.items.len(),
            title.clone().unwrap_or_default()
        ));

        let mut metadata = vec![];
        if let Some(title) = title {
            let line = Line::from(format!("Title: {}", title));
            metadata.push(line);
        }
        if let Some(date) = item.data.posted {
            let date = date.format(CONFIG.theme.date_format.as_str());
            let line = Line::from(format!("Posted: {date}"));
            metadata.push(line);
        }
        if !metadata.is_empty() {
            let line = Line::from("");
            metadata.push(line);
        }

        let content = item
            .data
            .content
            .or(item.data.summary)
            .or_else(|| {
                item.data
                    .media
                    .first()
                    .and_then(|c| c.0.description.as_ref())
                    .map(|a| a.content.clone())
            })
            .unwrap_or_default();

        let lines = content
            .lines()
            .map(|l| Line::from(l.to_string()))
            .collect::<Vec<Line>>();
        metadata.extend(lines);

        self.content = ScrollableParagraph::new(metadata);
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
            Event::Key(ev) => {
                if let Some(cmd) = self.keymap.get_command(ev) {
                    match cmd {
                        DetailedItemCommand::OpenItem => {
                            if let Some(link) = self.item().data.links.first()
                                && let Err(e) = open::that_detached(&link.0.href)
                            {
                                return AppRequest::OpenPopupView(e.to_string());
                            }
                        }
                        DetailedItemCommand::OpenLinks => {
                            return AppRequest::OpenLinksView(
                                Filter::new().item_id(self.item().data.id.clone()),
                            )
                        }
                        DetailedItemCommand::OpenItemInfo => {
                            return AppRequest::OpenPopupView(format!(
                                "TITLE: {}\nDATE: {}",
                                self.item().data.title.clone().unwrap_or_default(),
                                self.item().data.posted.unwrap_or_default(),
                            ))
                        }
                        DetailedItemCommand::PrevItem => {
                            self.item_idx = self.item_idx.saturating_sub(1).max(0);
                            return AppRequest::RefreshView;
                        }
                        DetailedItemCommand::NextItem => {
                            self.item_idx = (self.item_idx + 1).min(self.items.len() - 1);
                            return AppRequest::RefreshView;
                        }
                    }
                }
            }

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

#[derive(Debug, Clone)]
pub enum DetailedItemCommand {
    OpenItem,
    OpenLinks,
    OpenItemInfo,
    NextItem,
    PrevItem,
}
impl Display for DetailedItemCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct DetailedItemKeyMap(pub Vec<ShortCut<DetailedItemCommand>>);
impl KeyMap for DetailedItemKeyMap {
    type Command = DetailedItemCommand;
    fn get_shortcuts(&self) -> &[ShortCut<Self::Command>] {
        &self.0
    }
    fn default() -> Self {
        Self(Vec::from([
            ShortCut(
                DetailedItemCommand::OpenItem,
                vec![KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE)],
            ),
            ShortCut(
                DetailedItemCommand::OpenLinks,
                vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)],
            ),
            ShortCut(
                DetailedItemCommand::OpenItemInfo,
                vec![KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)],
            ),
            ShortCut(
                DetailedItemCommand::NextItem,
                vec![
                    KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
                ],
            ),
            ShortCut(
                DetailedItemCommand::PrevItem,
                vec![
                    KeyEvent::new(KeyCode::Char('K'), KeyModifiers::NONE),
                    KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
                ],
            ),
        ]))
    }
}
