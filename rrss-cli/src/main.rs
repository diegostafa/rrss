#![feature(let_chains)]
#![warn(unused_results)]

use clap::Parser;
use cli::{Cli, Commands, QueryCommand, QueryTarget};
use rrss_core::feed_manager::{FeedManager, TaskStatus};
use rrss_core::filter::Filter;
use rrss_core::sorter::Sorter;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut fm = FeedManager::new();

    match args.command {
        Commands::Dry => {}
        Commands::Fetch => {
            println!("fetching...");
            async_std::task::block_on(fm.update_feeds(&Filter::new(), || {}));
            match fm.poll_update_feeds() {
                TaskStatus::None | TaskStatus::Running => unreachable!(),
                TaskStatus::Error(e) => eprintln!("{e}"),
                TaskStatus::Done((errs, save_handle)) => {
                    println!("done");
                    if !errs.is_empty() {
                        eprintln!("{:?}", errs);
                    }
                    save_handle.join().expect("failed to save feeds");
                }
            }
        }
        Commands::Clear => fm.clear(),
        Commands::Query { query } => match query {
            QueryTarget::Feed { query } => match query {
                QueryCommand::Read => {
                    let res = fm.get_feeds(&Filter::new().read_feed(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.id());
                    }
                }
                QueryCommand::Unread => {
                    let res = fm.get_feeds(&Filter::new().unread_feed(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.id());
                    }
                }
                QueryCommand::All => {
                    let res = fm.get_feeds(&Filter::new(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.id());
                    }
                }
                QueryCommand::Tag { tag } => {
                    let res = fm.get_items(&Filter::new().tag_id(tag), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r);
                    }
                }
                QueryCommand::Contains { pattern } => {
                    let res = fm.get_items(&Filter::new().item_contains(pattern), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r);
                    }
                }
            },
            QueryTarget::Item { query } => match query {
                QueryCommand::Read => {
                    let res = fm.get_items(&Filter::new().read_item(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.data.id);
                    }
                }
                QueryCommand::Unread => {
                    let res = fm.get_items(&Filter::new().unread_item(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.data.id);
                    }
                }
                QueryCommand::All => {
                    let res = fm.get_items(&Filter::new(), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.data.id);
                    }
                }
                QueryCommand::Tag { tag } => {
                    let res = fm.get_items(&Filter::new().tag_id(tag), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r);
                    }
                }
                QueryCommand::Contains { pattern } => {
                    let res = fm.get_items(&Filter::new().item_contains(pattern), &Sorter::NONE);
                    for r in res {
                        println!("{:?}", r.data.title);
                    }
                }
            },
        },
    };

    Ok(())
}
