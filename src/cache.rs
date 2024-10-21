use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::globals::{CACHE_FILE, PROJECT_NAME};
use crate::model::adapters::FeedAdapter;
use crate::model::models::{Feed, FeedId, FeedMetrics};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableFeed {
    pub id: FeedId,
    pub data: FeedAdapter,
    pub metrics: FeedMetrics,
}
impl SerializableFeed {
    pub fn try_from_feed(feed: &Feed) -> Option<Self> {
        feed.data.as_ref().map(|data| Self {
            id: feed.id().clone(),
            data: data.clone(),
            metrics: feed.metrics.clone(),
        })
    }
}

pub struct CachedFeeds();
impl CachedFeeds {
    pub fn init() {
        let proj = ProjectDirs::from("", "", PROJECT_NAME).unwrap();
        fs::create_dir_all(proj.data_dir()).unwrap();
        let path = proj.data_dir().join(CACHE_FILE);
        if !path.exists() {
            fs::File::create(path).unwrap();
        }
    }

    pub fn save(feeds: &[SerializableFeed]) -> Result<(), std::io::Error> {
        let path = ProjectDirs::from("", "", PROJECT_NAME)
            .unwrap()
            .data_dir()
            .join(CACHE_FILE);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        file.write_all(&bincode::serialize(feeds).unwrap())?;
        Ok(())
    }
    pub fn load() -> Result<Vec<SerializableFeed>, std::io::Error> {
        let path = ProjectDirs::from("", "", PROJECT_NAME)
            .unwrap()
            .data_dir()
            .join(CACHE_FILE);

        match fs::File::open(path) {
            Ok(mut file) => {
                let mut data = vec![];
                file.read_to_end(&mut data)?;
                if data.is_empty() {
                    return Ok(Vec::new());
                }
                let data = bincode::deserialize(&data).unwrap();
                Ok(data)
            }
            Err(e) => panic!("{e}"),
        }
    }
}
