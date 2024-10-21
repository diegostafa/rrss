use lazy_static::lazy_static;
use ratatui_helpers::config::parse_toml;

use crate::config::{Config, PartialConfig};

pub const PROJECT_NAME: &str = "rrss";
pub const CACHE_FILE: &str = "feeds.bin";
pub const CONFIG_FILE: &str = "config.toml";
pub const SOURCES_FILE: &str = "sources.toml";

lazy_static! {
    pub static ref CONFIG: Config = parse_toml::<PartialConfig, _>(PROJECT_NAME, CONFIG_FILE);
}
