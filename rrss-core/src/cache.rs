use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

use directories::ProjectDirs;

use crate::globals::{CACHE_FILE, PROJECT_NAME};
use crate::models::Feed;

pub struct CachedFeeds;
impl CachedFeeds {
    pub fn init() {
        let proj = ProjectDirs::from("", "", PROJECT_NAME).unwrap();
        fs::create_dir_all(proj.data_dir()).unwrap();
        let path = proj.data_dir().join(CACHE_FILE);
        if !path.exists() {
            let _ = fs::File::create(path).unwrap();
        }
    }
    pub fn save(feeds: &[Feed]) -> Result<(), Box<dyn std::error::Error>> {
        let path = ProjectDirs::from("", "", PROJECT_NAME)
            .unwrap()
            .data_dir()
            .join(CACHE_FILE);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        file.write_all(&bincode::serde::encode_to_vec(
            feeds,
            bincode::config::legacy(),
        )?)?;
        Ok(())
    }
    pub fn load() -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
        let path = ProjectDirs::from("", "", PROJECT_NAME)
            .unwrap()
            .data_dir()
            .join(CACHE_FILE);

        match fs::File::open(path) {
            Ok(mut file) => {
                let mut data = vec![];
                let _ = file.read_to_end(&mut data)?;
                if data.is_empty() {
                    return Ok(Vec::new());
                }
                let data = bincode::serde::decode_from_slice(&data, bincode::config::legacy())?;
                Ok(data.0)
            }
            Err(e) => panic!("{e}"),
        }
    }
}
