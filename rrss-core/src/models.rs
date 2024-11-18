use std::fmt::{Debug, Display};
use std::hash::Hash;

use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use itertools::Itertools;
use ratatui::layout::{Alignment, Constraint};
use ratatui::style::Style;
use ratatui_helpers::stateful_table::Tabular;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

use super::sorter::Sorter;
use crate::config::{FeedFilter, FeedSource};
use crate::globals::CONFIG;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeedId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    pub conf: FeedSource,
    pub state: FeedState,
    pub data: Option<FeedData>,
}
impl Feed {
    pub fn refresh_items_state(&mut self) {
        if let Some(data) = &mut self.data {
            data.items.iter_mut().for_each(|i| {
                if let Some(filter) = &self.conf.filter {
                    i.state.is_filtered = filter.invert ^ i.title_matches(filter);
                }
            });
        }
    }
    pub fn merge_feed(&mut self, mut new: FeedData) {
        match &mut self.data {
            Some(old) => {
                new.items.retain(|i| !old.items.contains(i));
                old.items.extend(new.items);
                old.items.sort_by(Item::BY_POSTED_REV.0);
                old.items.truncate(self.conf.max_items as usize);
            }
            _ => self.data = Some(new),
        };
        self.refresh_items_state();
        self.refresh_feed_state();
    }
    pub fn url(&self) -> String {
        self.conf.url.0.to_string()
    }
    pub fn id(&self) -> &FeedId {
        &self.conf.url
    }
    pub fn feed_type(&self) -> FeedType {
        self.data
            .as_ref()
            .map_or(FeedType(None), |d| d.feed_type.clone())
    }
    pub fn items(&self) -> Option<&Vec<Item>> {
        self.data.as_ref().map(|d| &d.items)
    }
    pub fn items_mut(&mut self) -> Option<&mut Vec<Item>> {
        self.data.as_mut().map(|d| &mut d.items)
    }
    pub fn clear_data(&mut self) {
        self.data = None;
    }
    pub fn increment_hits(&mut self) {
        self.state.hits += 1;
    }
    pub fn refresh_feed_state(&mut self) {
        if let Some(feed) = &self.data {
            self.state.latest_item_date = feed.items.iter().map(|i| i.data.posted).max().flatten();
            self.state.is_recent = self.state.latest_item_date.map_or(false, |date| {
                (Utc::now() - date).num_days() < CONFIG.relative_time_threshold as i64
            })
        }
    }
    pub fn has_new_unfiltered(&self) -> bool {
        if self.conf.filter.is_none() {
            return false;
        }
        self.items().map_or(false, |i| {
            i.iter().any(|i| !i.state.is_read && !i.state.is_filtered)
        })
    }
    pub fn tot_unread(&self) -> usize {
        self.items()
            .map(|i| i.iter().filter(|i| !i.state.is_read).count())
            .unwrap_or_default()
    }
    pub fn name(&self) -> String {
        self.data
            .clone()
            .map(|d| d.title)
            .unwrap_or_else(|| self.id().0.clone())
    }
}
impl PartialEq for Feed {
    fn eq(&self, other: &Self) -> bool {
        self.conf == other.conf
    }
}
impl Tabular for Feed {
    type Value = FeedId;
    type ColumnValue = Sorter<Feed>;

    fn value(&self) -> Self::Value {
        self.id().clone()
    }
    fn column_values() -> Vec<Self::ColumnValue> {
        vec![
            Feed::BY_TOT_UNREADS,
            Feed::BY_TYPE,
            Feed::BY_TOT_UNREADS,
            Feed::BY_TITLE,
            Feed::BY_LATEST_ITEM,
            Feed::BY_HITS,
        ]
    }
    fn content(&self) -> Vec<String> {
        let empty = vec![];
        let items = self.items().unwrap_or(&empty);
        let tot_items = items.len();
        let tot_unread = self.tot_unread();

        let marker = match () {
            _ if self.has_new_unfiltered() => '*', // todo: add config option
            _ if tot_unread > 0 => CONFIG.theme.unread_marker,
            _ => CONFIG.theme.read_marker,
        };

        let latest_item_date = self
            .state
            .latest_item_date
            .map(pretty_date)
            .unwrap_or_default();

        vec![
            format!("{}", marker),
            format!("{}", self.feed_type()),
            format!("({}/{})", tot_unread, tot_items),
            format!("{}", self.name()),
            format!("{}", latest_item_date),
            format!("{}", self.state.hits),
        ]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![
            format!("N"),
            format!("Type"),
            format!("U/T"),
            format!("Title"),
            format!("Latest"),
            format!("Hits"),
        ])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![
            Constraint::Length,
            Constraint::Length,
            Constraint::Length,
            Constraint::Fill,
            Constraint::Min,
            Constraint::Min,
        ]
    }
    fn style(&self) -> Style {
        let mut style = Style::default();
        if self.has_new_unfiltered() {
            style = style.fg(ratatui::style::Color::LightCyan); // todo: add config option
        }
        if self.tot_unread() > 0 {
            style = style.fg(CONFIG.theme.fg_unread_color);
        }
        if self.state.is_recent {
            style = style.fg(ratatui::style::Color::LightGreen);
        }
        style
    }
    fn column_alignments() -> Option<Vec<Alignment>> {
        Some(vec![
            Alignment::Left,
            Alignment::Left,
            Alignment::Right,
            Alignment::Left,
            Alignment::Left,
            Alignment::Left,
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedState {
    pub latest_item_date: Option<DateTime<Utc>>,
    pub hits: usize,
    pub is_recent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedData {
    pub feed_type: FeedType,
    pub title: String,
    pub items: Vec<Item>,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub links: Vec<Link>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub language: Option<String>,
}
impl FeedData {
    pub fn from(feed: feed_rs::model::Feed, url: &str) -> Self {
        Self {
            feed_type: FeedType(Some(feed.feed_type)),
            title: feed.title.map(|t| t.content).unwrap_or_default(),
            items: feed
                .entries
                .into_iter()
                .map(|i| Item {
                    data: ItemData::from(i, url),
                    state: ItemState {
                        is_read: false,
                        is_filtered: false,
                    },
                })
                .collect_vec(),
            published: feed.published,
            updated: feed.updated,
            links: feed.links.into_iter().map(Link).collect(),
            authors: feed.authors.into_iter().map(|a| a.name).collect(),
            description: feed.description.map(|d| d.content),
            categories: feed.categories.into_iter().map(|c| c.term).collect(),
            language: feed.language,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub String, pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub data: ItemData,
    pub state: ItemState,
}
impl Item {
    pub fn title_matches(&self, filter: &FeedFilter) -> bool {
        if let Some(title) = &self.data.title {
            return RegexBuilder::new(&filter.pattern)
                .case_insensitive(filter.case_insensitive)
                .build()
                .unwrap()
                .is_match(title);
        }
        false
    }
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.data.id == other.data.id
    }
}
impl Eq for Item {}
impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.id.hash(state);
    }
}
impl Tabular for Item {
    type Value = ItemId;
    type ColumnValue = Sorter<Item>;

    fn column_values() -> Vec<Self::ColumnValue> {
        vec![Item::BY_IS_READ, Item::BY_TITLE, Item::BY_POSTED]
    }
    fn value(&self) -> Self::Value {
        self.data.id.clone()
    }
    fn content(&self) -> Vec<String> {
        let marker = match self.state.is_read {
            true => CONFIG.theme.read_marker,
            _ => CONFIG.theme.unread_marker,
        };
        vec![
            format!("{}", marker),
            format!("{}", self.data.title.clone().unwrap_or_default()),
            format!("{}", self.data.posted.map(pretty_date).unwrap_or_default()),
        ]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![format!("N"), format!("Title"), format!("Posted")])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![Constraint::Length, Constraint::Fill, Constraint::Min]
    }
    fn style(&self) -> Style {
        let mut style = Style::default();
        if !self.state.is_read {
            style = style
                .fg(CONFIG.theme.fg_unread_color)
                .bg(CONFIG.theme.bg_unread_color);
        }
        if self.state.is_filtered {
            style = style
                .fg(CONFIG.theme.fg_filtered_color)
                .bg(CONFIG.theme.bg_filterd_color);
        }
        style
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemState {
    pub is_read: bool,
    pub is_filtered: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemData {
    pub id: ItemId,
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub media: Vec<MediaObject>,
    pub posted: Option<DateTime<Utc>>,
    pub links: Vec<Link>,
}
impl ItemData {
    fn from(item: feed_rs::model::Entry, feed_url: &str) -> Self {
        Self {
            id: ItemId(feed_url.to_string(), item.id),
            title: item.title.map(|t| t.content),
            content: item
                .content
                .and_then(|s| s.body)
                .as_deref()
                .map(html_to_text),
            summary: item.summary.map(|s| html_to_text(&s.content)),
            posted: item.published.or(item.updated),
            links: item.links.into_iter().map(Link).collect(),
            media: item.media.into_iter().map(MediaObject).collect(),
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    pub name: String,
    pub count: usize,
}
impl Tabular for Tag {
    type Value = String;
    type ColumnValue = Sorter<Tag>;
    fn column_values() -> Vec<Self::ColumnValue> {
        vec![Tag::BY_NAME, Tag::BY_COUNT]
    }
    fn value(&self) -> Self::Value {
        self.name.clone()
    }
    fn content(&self) -> Vec<String> {
        vec![format!("{}", self.name), format!("{}", self.count)]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![format!("Name"), format!("Count")])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![Constraint::Length, Constraint::Length]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link(pub feed_rs::model::Link);
impl Tabular for Link {
    type Value = String;
    type ColumnValue = ();
    fn column_values() -> Vec<Self::ColumnValue> {
        vec![]
    }

    fn value(&self) -> Self::Value {
        self.0.href.clone()
    }
    fn content(&self) -> Vec<String> {
        vec![
            format!("{}", self.0.title.clone().unwrap_or_default()),
            format!("{}", self.0.media_type.clone().unwrap_or_default()),
            format!("{}", self.0.href),
        ]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![
            format!("Title"),
            format!("Mime type"),
            format!("Link"),
        ])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![Constraint::Length, Constraint::Length, Constraint::Fill]
    }

    fn style(&self) -> Style {
        Style::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaContent(pub feed_rs::model::MediaContent);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaObject(pub feed_rs::model::MediaObject);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedType(pub Option<feed_rs::model::FeedType>);
impl Display for FeedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "Unknown"),
            Some(feed_rs::model::FeedType::Atom) => write!(f, "Atom"),
            Some(feed_rs::model::FeedType::JSON) => write!(f, "JSON"),
            Some(feed_rs::model::FeedType::RSS0) => write!(f, "RSS0"),
            Some(feed_rs::model::FeedType::RSS1) => write!(f, "RSS1"),
            Some(feed_rs::model::FeedType::RSS2) => write!(f, "RSS2"),
        }
    }
}

fn pretty_date(date: DateTime<Utc>) -> String {
    let delta_days = (Utc::now() - date).num_days();
    match delta_days {
        0 => HumanTime::from(date).to_text_en(Accuracy::Rough, Tense::Past),
        _ if delta_days < CONFIG.relative_time_threshold as i64 => {
            format!("{}, {}", HumanTime::from(date), date.format("%a, %H:%M"))
        }
        _ => date.format(CONFIG.theme.date_format.as_str()).to_string(),
    }
}

fn html_to_text(html: &str) -> String {
    html2text::config::plain()
        .raw_mode(true)
        .no_table_borders()
        .string_from_read(html.as_bytes(), 1000)
        .unwrap()
        .trim()
        .to_string()
}
