#![feature(let_chains)]

use cache::CachedFeeds;
use clap::Parser;
use cli::{Cli, Commands, QueryCommand};
use config::{FromPartialToml, Sources};
use feed_manager::FeedManager;
use globals::SOURCES_FILE;
use model::filter::Filter;
use model::sorter::Sorter;
use tui::app::App;

pub mod cache;
pub mod cli;
pub mod config;
pub mod feed_manager;
pub mod globals;
pub mod model;
pub mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CachedFeeds::init();
    let args = Cli::parse();
    let sources = Sources::parse(SOURCES_FILE)?;
    let mut fm = FeedManager::new(sources);

    match args.command {
        Commands::Dry => {}
        Commands::Update => {
            async_std::task::block_on(fm.update_feeds(&Filter::default()));
            fm.poll_update_feeds();
        }
        Commands::Tui => App::new(fm).init().run()?,
        Commands::Clear => fm.clear_items(),
        Commands::Query { query } => match query {
            QueryCommand::ReadCount => {
                let items = fm.get_items(&Filter::default().is_item_read(), &Sorter::NONE);
                println!("read: {}", items.len());
            }
            QueryCommand::UnreadCount => {
                let items = fm.get_items(&Filter::default().is_item_unread(), &Sorter::NONE);
                println!("unread: {}", items.len());
            }
        },
    };

    Ok(())
}
