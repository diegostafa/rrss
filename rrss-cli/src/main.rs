#![feature(let_chains)]
#![warn(unused_results)]

use clap::Parser;
use cli::{Cli, Commands, QueryCommand, QueryCommandTarget};
use rrss_core::feed_manager::{FeedManager, TaskStatus};
use rrss_core::model::filter::Filter;
use rrss_core::model::sorter::Sorter;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut fm = FeedManager::new();

    match args.command {
        Commands::Dry => {}
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
        Commands::Clear => fm.clear(),
        Commands::Query { query } => match query {
            QueryCommandTarget::Feed { query } => match query {
                QueryCommand::Read => {
                    let res = fm.get_feeds(&Filter::new().read_feed(), &Sorter::NONE);
                    println!("{:?}", res);
                }
                QueryCommand::Unread => {
                    let res = fm.get_feeds(&Filter::new().unread_feed(), &Sorter::NONE);
                    println!("{:?}", res);
                }
                QueryCommand::ReadCount => {
                    let res = fm.get_feeds(&Filter::new().read_feed(), &Sorter::NONE);
                    println!("{:?}", res.len());
                }
                QueryCommand::UnreadCount => {
                    let res = fm.get_feeds(&Filter::new().unread_feed(), &Sorter::NONE);
                    println!("{:?}", res.len());
                }
            },
            QueryCommandTarget::Item { query } => match query {
                QueryCommand::Read => {
                    let res = fm.get_items(&Filter::new().read_item(), &Sorter::NONE);
                    println!("{:?}", res);
                }
                QueryCommand::Unread => {
                    let res = fm.get_items(&Filter::new().unread_item(), &Sorter::NONE);
                    println!("{:?}", res);
                }
                QueryCommand::ReadCount => {
                    let res = fm.get_items(&Filter::new().read_item(), &Sorter::NONE);
                    println!("{:?}", res);
                }
                QueryCommand::UnreadCount => {
                    let res = fm.get_items(&Filter::new().unread_item(), &Sorter::NONE);
                    println!("{:?}", res);
                }
            },
        },
    };

    Ok(())
}
