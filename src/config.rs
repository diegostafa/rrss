use std::hash::Hash;
use std::{fs, io};

use directories::ProjectDirs;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::globals::PROJECT_NAME;
use crate::model::models::FeedId;

pub trait FromPartialToml: Sized {
    type Partial: DeserializeOwned;

    fn partial_to_full(val: Self::Partial) -> Self;
    fn parse(file: &str) -> Result<Self, io::Error> {
        let proj = ProjectDirs::from("", "", PROJECT_NAME).unwrap();
        let file = proj.config_dir().join(file);
        let toml = toml::from_str(&fs::read_to_string(file)?);
        match toml {
            Ok(toml) => Ok(Self::partial_to_full(toml)),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct PartialKeybinds {
    pub cancel: Option<Vec<String>>,
    pub submit: Option<Vec<String>>,
}
#[derive(Deserialize, Default, Debug)]
pub struct Keybinds {
    pub cancel: Vec<String>,
    pub submit: Vec<String>,
}

#[derive(Deserialize)]
pub struct PartialConfig {
    dim_filtered_items: Option<bool>,
    date_format: Option<String>,
    max_days_until_old: Option<u32>,
    unread_marker: Option<String>,
    read_marker: Option<String>,
    max_concurrency: Option<usize>,
    keybinds: Option<Keybinds>,
}
#[derive(Deserialize)]
pub struct Config {
    pub date_format: String,
    pub max_days_until_old: u32,
    pub dim_filtered_items: bool,
    pub unread_marker: String,
    pub read_marker: String,
    pub max_concurrency: usize,
    pub keybinds: Keybinds,
}
impl FromPartialToml for Config {
    type Partial = PartialConfig;

    fn partial_to_full(val: PartialConfig) -> Self {
        Self {
            dim_filtered_items: val.dim_filtered_items.unwrap_or(false),
            date_format: val.date_format.unwrap_or_else(|| "%Y-%m-%d".to_string()),
            unread_marker: val.unread_marker.unwrap_or_else(|| String::from("*")),
            read_marker: val.read_marker.unwrap_or_else(|| String::from(" ")),
            max_concurrency: val.max_concurrency.unwrap_or(5),
            keybinds: val.keybinds.unwrap_or_default(),
            max_days_until_old: val.max_days_until_old.unwrap_or(3),
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
            max_items: value.max_items.unwrap_or(10000),
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
impl FromPartialToml for Sources {
    type Partial = PartialSources;

    fn partial_to_full(val: PartialSources) -> Self {
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
