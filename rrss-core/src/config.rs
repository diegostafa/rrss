use std::hash::Hash;
use std::str::FromStr;

use itertools::Itertools;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::cache::SerializableFeed;
use crate::model::models::{Feed, FeedId};

#[derive(Deserialize, Default)]
pub struct PartialTheme {
    fg_header_color: Option<String>,
    fg_selected_color: Option<String>,
    fg_normal_color: Option<String>,
    fg_unread_color: Option<String>,
    fg_filtered_color: Option<String>,
    fg_item_header: Option<String>,

    bg_header_color: Option<String>,
    bg_selected_color: Option<String>,
    bg_normal_color: Option<String>,
    bg_unread_color: Option<String>,
    bg_filterd_color: Option<String>,
    bg_item_header: Option<String>,

    column_spacing: Option<u16>,
    border_color: Option<String>,
    borders: Option<bool>,
    rounded_borders: Option<bool>,
    date_format: Option<String>,
    unread_marker: Option<char>,
    read_marker: Option<char>,
}
#[derive(Debug)]
pub struct Theme {
    pub fg_header_color: Color,
    pub fg_selected_color: Color,
    pub fg_normal_color: Color,
    pub fg_unread_color: Color,
    pub fg_filtered_color: Color,
    pub fg_item_header: Color,

    pub bg_header_color: Color,
    pub bg_selected_color: Color,
    pub bg_normal_color: Color,
    pub bg_unread_color: Color,
    pub bg_filterd_color: Color,
    pub bg_item_header: Color,

    pub column_spacing: u16,
    pub border_color: Color,
    pub borders: bool,
    pub rounded_borders: bool,
    pub date_format: String,
    pub unread_marker: char,
    pub read_marker: char,
}
impl From<PartialTheme> for Theme {
    fn from(val: PartialTheme) -> Self {
        Self {
            fg_header_color: Color::from_str(&val.fg_header_color.unwrap_or("blue".to_string()))
                .unwrap(),
            fg_selected_color: Color::from_str(&val.fg_selected_color.unwrap_or("white".into()))
                .unwrap(),
            fg_normal_color: Color::from_str(&val.fg_normal_color.unwrap_or("white".into()))
                .unwrap(),
            fg_unread_color: Color::from_str(&val.fg_unread_color.unwrap_or("yellow".into()))
                .unwrap(),
            fg_filtered_color: Color::from_str(&val.fg_filtered_color.unwrap_or("darkgray".into()))
                .unwrap(),
            fg_item_header: Color::from_str(&val.fg_item_header.unwrap_or("white".into())).unwrap(),

            bg_header_color: Color::from_str(&val.bg_header_color.unwrap_or("black".into()))
                .unwrap(),
            bg_selected_color: Color::from_str(&val.bg_selected_color.unwrap_or("darkgray".into()))
                .unwrap(),
            bg_normal_color: Color::from_str(&val.bg_normal_color.unwrap_or("black".into()))
                .unwrap(),
            bg_unread_color: Color::from_str(&val.bg_unread_color.unwrap_or("black".into()))
                .unwrap(),
            bg_filterd_color: Color::from_str(&val.bg_filterd_color.unwrap_or("black".into()))
                .unwrap(),
            bg_item_header: Color::from_str(&val.bg_item_header.unwrap_or("blue".into())).unwrap(),

            border_color: Color::from_str(&val.border_color.unwrap_or("yellow".into())).unwrap(),
            borders: val.borders.unwrap_or(true),
            rounded_borders: val.rounded_borders.unwrap_or(false),
            date_format: val.date_format.unwrap_or_else(|| "%Y-%m-%d".to_string()),
            unread_marker: val.unread_marker.unwrap_or('*'),
            read_marker: val.read_marker.unwrap_or(' '),
            column_spacing: val.column_spacing.unwrap_or(1),
        }
    }
}

#[derive(Deserialize)]
pub struct PartialConfig {
    relative_time_threshold: Option<u32>,
    max_concurrency: Option<usize>,
    theme: Option<PartialTheme>,
}
pub struct Config {
    pub relative_time_threshold: u32,
    pub max_concurrency: usize,
    pub theme: Theme,
}
impl From<PartialConfig> for Config {
    fn from(val: PartialConfig) -> Self {
        Self {
            max_concurrency: val.max_concurrency.unwrap_or(5),
            relative_time_threshold: val.relative_time_threshold.unwrap_or(3),
            theme: Theme::from(val.theme.unwrap_or_default()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PartialFeedFilter {
    pattern: String,
    invert: Option<bool>,
    case_insensitive: Option<bool>,
}
#[derive(Debug, Clone)]
pub struct FeedFilter {
    pub pattern: String,
    pub invert: bool,
    pub case_insensitive: bool,
}
impl From<PartialFeedFilter> for FeedFilter {
    fn from(value: PartialFeedFilter) -> Self {
        Self {
            pattern: value.pattern,
            invert: value.invert.unwrap_or(false),
            case_insensitive: value.case_insensitive.unwrap_or(false),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PartialFeedSource {
    url: Option<FeedId>,
    tags: Vec<String>,
    manual_update: Option<bool>,
    filter: Option<PartialFeedFilter>,
    max_items: Option<u32>,
}
#[derive(Debug, Clone)]
pub struct FeedSource {
    pub url: FeedId,
    pub tags: Vec<String>,
    pub manual_update: bool,
    pub filter: Option<FeedFilter>,
    pub max_items: u32,
}
impl From<PartialFeedSource> for FeedSource {
    fn from(value: PartialFeedSource) -> Self {
        Self {
            url: value.url.expect("url is required"),
            tags: value.tags,
            manual_update: value.manual_update.unwrap_or(false),
            filter: value.filter.map(FeedFilter::from),
            max_items: value.max_items.unwrap_or(5000),
        }
    }
}
impl PartialEq for FeedSource {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for FeedSource {}
impl Hash for FeedSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

#[derive(Deserialize)]
pub struct PartialSources {
    sources: Option<Vec<PartialFeedSource>>,
}
pub struct Sources(pub Vec<FeedSource>);
impl Sources {
    pub fn bind_to_cached(self, cached_feeds: Vec<SerializableFeed>) -> Vec<Feed> {
        self.0
            .into_iter()
            .map(|s| {
                let feed = cached_feeds.iter().find(|f| f.id == s.url);
                let mut feed = Feed {
                    conf: s.clone(),
                    data: feed.map(|f| f.data.clone()),
                    metrics: feed.map(|f| f.metrics.clone()).unwrap_or_default(),
                };
                feed.refresh_items_metrics();
                feed
            })
            .collect()
    }
}
impl From<PartialSources> for Sources {
    fn from(val: PartialSources) -> Self {
        if let Some(sources) = val.sources {
            let sources = sources.into_iter().map(FeedSource::from).collect_vec();
            let prev_size = sources.len();
            let uniques = sources.iter().cloned().unique().collect_vec();
            if prev_size != uniques.len() {
                for (src, freq) in sources.into_iter().counts() {
                    if freq > 1 {
                        println!("[warning] duplicate source: {:?}", src.url);
                    }
                }
            }
            return Sources(uniques);
        }
        Sources(vec![])
    }
}
