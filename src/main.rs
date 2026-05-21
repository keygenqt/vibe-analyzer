//! Vibe Analyzer - Main entry point.
//! Parses CLI arguments and dispatches to the appropriate subcommands (add, del, list, scan).

use std::path::PathBuf;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::analyze::AnalyzeAction;
use crate::cli::serve::ServeAction;
use crate::cli::source::SourceAction;
use crate::cli::stats::StatsAction;
use crate::configs::app::AppConfig;

mod analyzer;
mod cli;
mod configs;
mod enrich;
mod export;
mod indexer;
mod mcp;
mod models;
mod opensearch;
mod scanner;
mod utils;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    /// Path to config file (default: ~/.vibe-analyzer/config.json5).
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan, enrich, export, and index sources.
    Analyze {
        #[command(subcommand)]
        action: AnalyzeAction,
    },

    /// Manage sources (add, remove, list, update)
    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

    /// Show sources information and verify indexed data.
    Stats {
        #[command(subcommand)]
        action: StatsAction,
    },

    /// Start MCP server for AI assistants integration
    Serve {
        #[command(subcommand)]
        action: ServeAction,
    },
}

#[tokio::main]
async fn main() {
    let app = App::parse();

    // Load config with override
    AppConfig::set_global_config(app.config.clone());

    match app.command {
        Some(Commands::Source { action }) => cli::source::run(action),
        Some(Commands::Analyze { action }) => cli::analyze::run(action).await,
        Some(Commands::Stats { action }) => cli::stats::run(action).await,
        Some(Commands::Serve { action }) => cli::serve::run(action).await,
        None => {
            let _ = App::command().print_help();
            print_newline!();
        }
    }
}
