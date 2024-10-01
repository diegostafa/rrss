use lazy_static::lazy_static;

use crate::config::{Config, FromPartialToml};

pub const PROJECT_NAME: &str = "rrss";
pub const CACHE_FILE: &str = "feeds.bin";
pub const CONFIG_FILE: &str = "config.toml";
pub const SOURCES_FILE: &str = "sources.toml";

lazy_static! {
    pub static ref CONFIG: Config = Config::parse(CONFIG_FILE).unwrap();
}
