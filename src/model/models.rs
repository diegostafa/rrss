use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style, Stylize};
use ratatui_helpers::stateful_table::Tabular;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

use super::adapters::{FeedAdapter, FeedTypeAdapter, MediaObjectAdapter};
use crate::config::{FeedFilter, FeedSource};
use crate::globals::CONFIG;
use crate::model::format_date;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedMetrics {
    pub latest_item_date: Option<DateTime<Utc>>,
    pub hits: usize,
    pub is_recent: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeedId(pub String);
impl Display for FeedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Clone)]
pub struct Feed {
    pub conf: FeedSource,
    pub metrics: FeedMetrics,
    pub data: Option<FeedAdapter>,
}
impl Feed {
    pub fn refresh_items_metrics(&mut self) {
        if let Some(data) = &mut self.data {
            data.items.iter_mut().for_each(|i| {
                if let Some(filter) = &self.conf.filter {
                    i.is_filtered = filter.invert ^ i.title_matches(filter);
                }
            });
        }
    }
    pub fn merge_feed(&mut self, new: FeedAdapter) {
        match &mut self.data {
            Some(old) => {
                old.items = old
                    .items
                    .clone()
                    .into_iter()
                    .chain(new.items)
                    .unique()
                    .take(self.conf.max_items as usize)
                    .collect();
            }
            _ => self.data = Some(new),
        };
        self.refresh_items_metrics();
        self.refresh_feed_metrics();
    }
    pub fn url(&self) -> &str {
        &self.conf.url.0
    }
    pub fn id(&self) -> &FeedId {
        &self.conf.url
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.conf.tags
    }
    pub fn feed_type(&self) -> FeedTypeAdapter {
        self.data
            .as_ref()
            .map_or(FeedTypeAdapter::Unknown, |d| d.feed_type.clone())
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
        self.metrics.hits += 1;
    }
    pub fn refresh_feed_metrics(&mut self) {
        if let Some(feed) = &self.data {
            self.metrics.latest_item_date = feed.items.iter().map(|i| i.posted).max().flatten();
            self.metrics.is_recent = self.metrics.latest_item_date.map_or(false, |date| {
                (Utc::now() - date).num_days() < CONFIG.relative_time_threshold as i64
            })
        }
    }
    pub fn tot_unread(&self) -> usize {
        self.items()
            .map(|i| i.iter().filter(|i| !i.is_read).count())
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

    fn value(&self) -> Self::Value {
        self.id().clone()
    }
    fn content(&self) -> Vec<String> {
        let tot_items = self.items().map(Vec::len).unwrap_or_default();
        let tot_unread = self.tot_unread();
        let unread_marker = match tot_unread > 0 {
            true => CONFIG.theme.unread_marker,
            _ => CONFIG.theme.read_marker,
        };

        let latest_item_date = self
            .metrics
            .latest_item_date
            .map(format_date)
            .unwrap_or_default();

        vec![
            format!("{}", unread_marker),
            format!("{}", self.feed_type()),
            format!("{}/{}", tot_unread, tot_items),
            format!("{}", self.name()),
            format!("{}", latest_item_date),
            format!("{}", self.metrics.hits),
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
        if self.tot_unread() > 0 {
            style = style.fg(Color::from_str(&CONFIG.theme.fg_unread_color).unwrap());
        }
        if self.metrics.is_recent {
            style = style.italic();
        }
        style
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);
impl Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: ItemId,
    pub is_read: bool,
    pub is_filtered: bool,
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub media: Vec<MediaObjectAdapter>,
    pub posted: Option<DateTime<Utc>>,
    pub links: Vec<Link>,
}
impl Item {
    pub fn url(&self) -> Option<String> {
        self.links.first().map(|l| l.href.clone())
    }
    pub fn title_matches(&self, filter: &FeedFilter) -> bool {
        if let Some(title) = &self.title {
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
        self.id == other.id
    }
}
impl Eq for Item {}
impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Tabular for Item {
    type Value = ItemId;
    fn value(&self) -> Self::Value {
        self.id.clone()
    }
    fn content(&self) -> Vec<String> {
        let marker = match self.is_read {
            true => CONFIG.theme.read_marker,
            _ => CONFIG.theme.unread_marker,
        };
        vec![
            format!("{}", marker),
            format!("{}", self.title.clone().unwrap_or_default()),
            format!("{}", self.posted.map(format_date).unwrap_or_default()),
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
        if !self.is_read {
            style = style
                .fg(Color::from_str(&CONFIG.theme.fg_unread_color).unwrap())
                .bg(Color::from_str(&CONFIG.theme.bg_unread_color).unwrap());
        }
        if self.is_filtered {
            style = style
                .fg(Color::from_str(&CONFIG.theme.fg_filtered_color).unwrap())
                .bg(Color::from_str(&CONFIG.theme.bg_filterd_color).unwrap());
        }
        style
    }
}

#[derive(Clone)]
pub struct Tag {
    pub name: String,
    pub count: usize,
}
impl Tabular for Tag {
    type Value = String;
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
pub struct Link {
    pub href: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
}
impl Tabular for Link {
    type Value = String;

    fn value(&self) -> Self::Value {
        self.href.clone()
    }
    fn content(&self) -> Vec<String> {
        vec![
            format!("{}", self.title.clone().unwrap_or_default()),
            format!("{}", self.mime_type.clone().unwrap_or_default()),
            format!("{}", self.href),
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

pub struct Shortcut {
    pub name: String,
    pub shortcut: Vec<String>,
}
impl Tabular for Shortcut {
    type Value = String;

    fn value(&self) -> Self::Value {
        self.name.clone()
    }
    fn content(&self) -> Vec<String> {
        vec![self.name.clone(), self.shortcut.iter().join(",")]
    }
    fn column_names() -> Option<Vec<String>> {
        Some(vec![format!("Name"), format!("Shortcut")])
    }
    fn column_constraints() -> Vec<fn(u16) -> Constraint> {
        vec![Constraint::Length, Constraint::Length]
    }
}
