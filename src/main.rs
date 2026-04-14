//! Vibe Analyzer - Main entry point.
//! Parses CLI arguments and dispatches to the appropriate subcommands (add, del, list, scan).

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::scan::ScanAction;
use crate::cli::search::SearchAction;
use crate::cli::serve::ServeAction;
use crate::cli::source::SourceAction;

mod analyzer;
mod cli;
mod configs;
mod export;
mod mcp;
mod opensearch;
mod scanner;
mod utils;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and extract structure from knowledge sources.
    Scan {
        #[command(subcommand)]
        action: ScanAction,
    },

    /// Search across indexed knowledge base
    Search {
        #[command(subcommand)]
        action: SearchAction,
    },

    /// Manage knowledge sources (add, remove, list, update)
    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

    /// Start MCP server for AI assistants integration
    Serve {
        #[command(subcommand)]
        action: ServeAction,
    },
}

#[tokio::main]
async fn main() {
    match App::parse().command {
        Some(Commands::Source { action }) => cli::source::run(action),
        Some(Commands::Scan { action }) => cli::scan::run(action).await,
        Some(Commands::Search { action }) => cli::search::run(action).await,
        Some(Commands::Serve { action }) => cli::serve::run(action).await,
        None => {
            let _ = App::command().print_help();
            print_newline!();
        }
    }
}
