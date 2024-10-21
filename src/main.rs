#![feature(let_chains)]

use cache::CachedFeeds;
use clap::Parser;
use cli::{Cli, Commands, QueryCommand};
use config::PartialSources;
use feed_manager::{FeedManager, TaskStatus};
use globals::{CONFIG, PROJECT_NAME, SOURCES_FILE};
use model::filter::Filter;
use model::sorter::Sorter;
use ratatui_helpers::config::parse_toml;
use tui::app::App;

mod cache;
mod cli;
mod config;
mod feed_manager;
mod globals;
mod model;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CachedFeeds::init();
    let args = Cli::parse();
    let sources = parse_toml::<PartialSources, _>(PROJECT_NAME, SOURCES_FILE);
    let mut fm = FeedManager::new(sources);

    match args.command {
        Commands::Dry => {
            CONFIG.max_concurrency; // force loading
        }
        Commands::Fetch => {
            async_std::task::block_on(fm.update_feeds(&Filter::new(), || {}));
            match fm.poll_update_feeds() {
                TaskStatus::None | TaskStatus::Running => unreachable!(),
                TaskStatus::Error(e) => eprintln!("{e}"),
                TaskStatus::Done((errs, save_handle)) => {
                    eprintln!("{:?}", errs);
                    save_handle.join().expect("failed to save feeds");
                }
            }
        }
        Commands::Tui => App::new(fm).init().run()?,
        Commands::Clear => fm.clear_items(),
        Commands::Query { query } => match query {
            QueryCommand::ReadCount => {
                let items = fm.get_items(&Filter::new().read_item(), &Sorter::NONE);
                println!("read: {}", items.len());
            }
            QueryCommand::UnreadCount => {
                let items = fm.get_items(&Filter::new().unread_item(), &Sorter::NONE);
                println!("unread: {}", items.len());
            }
        },
    };

    Ok(())
}
