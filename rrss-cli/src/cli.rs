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
        query: QueryTarget,
    },
}

#[derive(Subcommand, Debug)]
pub enum QueryTarget {
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
    All,
    Read,
    Unread,
    Tag { tag: String },
    Contains { pattern: String },
}
