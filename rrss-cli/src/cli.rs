use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    Dry,
    // DumpConfig,
    // DumpSources,
    Fetch,
    Clear,
    Query {
        #[command(subcommand)]
        query: QueryCommandTarget,
    },
}

#[derive(Subcommand, Debug)]
pub enum QueryCommandTarget {
    Feed {
        #[command(subcommand)]
        query: QueryCommand,
    },
    Item {
        #[command(subcommand)]
        query: QueryCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum QueryCommand {
    Read,
    Unread,
    ReadCount,
    UnreadCount,
}

// rrss query feed read-count
// rrss query feed tag "rust"
