#![feature(let_chains)]
#![warn(unused_results)]
#![allow(clippy::single_match)]
#![allow(clippy::useless_format)]

use app::App;
use rrss_core::feed_manager::FeedManager;

pub mod app;
pub mod keymaps;
pub mod theme;
pub mod views;
pub mod widgets;

fn main() {
    App::new(FeedManager::new()).init().run().unwrap()
}
