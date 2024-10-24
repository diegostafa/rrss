use std::hash::Hash;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::cache::SerializableFeed;
use crate::model::models::{Feed, FeedId};

#[derive(Deserialize, Default)]
pub struct PartialTheme {
    fg_header_color: Option<String>,
    fg_selected_color: Option<String>,
    fg_normal_color: Option<String>,
    fg_unread_color: Option<String>,
    fg_read_color: Option<String>,
    fg_filtered_color: Option<String>,

    bg_header_color: Option<String>,
    bg_selected_color: Option<String>,
    bg_normal_color: Option<String>,
    bg_unread_color: Option<String>,
    bg_read_color: Option<String>,
    bg_filterd_color: Option<String>,

    column_spacing: Option<u16>,
    border_color: Option<String>,
    borders: Option<bool>,
    rounded_borders: Option<bool>,
    scrollbars: Option<bool>,
    date_format: Option<String>,
    unread_marker: Option<char>,
    read_marker: Option<char>,
}
#[derive(Deserialize, Debug)]
pub struct Theme {
    pub fg_header_color: String,
    pub fg_selected_color: String,
    pub fg_normal_color: String,
    pub fg_unread_color: String,
    pub fg_read_color: String,
    pub fg_filtered_color: String,

    pub bg_header_color: String,
    pub bg_selected_color: String,
    pub bg_normal_color: String,
    pub bg_unread_color: String,
    pub bg_read_color: String,
    pub bg_filterd_color: String,

    pub column_spacing: u16,
    pub border_color: String,
    pub borders: bool,
    pub rounded_borders: bool,
    pub scrollbars: bool,
    pub date_format: String,
    pub unread_marker: char,
    pub read_marker: char,
}
impl From<PartialTheme> for Theme {
    fn from(val: PartialTheme) -> Self {
        Self {
            fg_header_color: val.fg_header_color.unwrap_or("blue".to_string()),
            fg_selected_color: val.fg_selected_color.unwrap_or("white".to_string()),
            fg_normal_color: val.fg_normal_color.unwrap_or("white".to_string()),
            fg_unread_color: val.fg_unread_color.unwrap_or("yellow".to_string()),
            fg_read_color: val.fg_read_color.unwrap_or("white".to_string()),
            fg_filtered_color: val.fg_filtered_color.unwrap_or("darkgray".to_string()),

            bg_header_color: val.bg_header_color.unwrap_or("black".to_string()),
            bg_selected_color: val.bg_selected_color.unwrap_or("darkgray".to_string()),
            bg_normal_color: val.bg_normal_color.unwrap_or("black".to_string()),
            bg_unread_color: val.bg_unread_color.unwrap_or("black".to_string()),
            bg_read_color: val.bg_read_color.unwrap_or("black".to_string()),
            bg_filterd_color: val.bg_filterd_color.unwrap_or("black".to_string()),

            border_color: val.border_color.unwrap_or("yellow".to_string()),
            borders: val.borders.unwrap_or(true),
            rounded_borders: val.rounded_borders.unwrap_or(false),
            date_format: val.date_format.unwrap_or_else(|| "%Y-%m-%d".to_string()),
            unread_marker: val.unread_marker.unwrap_or('*'),
            read_marker: val.read_marker.unwrap_or(' '),
            scrollbars: val.scrollbars.unwrap_or(false),
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
#[derive(Deserialize)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Deserialize)]
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
