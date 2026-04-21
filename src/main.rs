//! Vibe Analyzer - Main entry point.
//! Parses CLI arguments and dispatches to the appropriate subcommands (add, del, list, scan).

use std::path::PathBuf;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::scan::ScanAction;
use crate::cli::serve::ServeAction;
use crate::cli::source::SourceAction;
use crate::cli::stats::StatsAction;
use crate::configs::app::AppConfig;

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
    /// Path to config directory (overrides default ~/.vibe-analyzer)
    #[arg(long, global = true)]
    config_dir: Option<PathBuf>,

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

    /// Show project information and verify indexed data.
    Stats {
        #[command(subcommand)]
        action: StatsAction,
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
    let app = App::parse();

    // Load config with override
    AppConfig::set_global_config_dir(app.config_dir.clone());

    match app.command {
        Some(Commands::Source { action }) => cli::source::run(action),
        Some(Commands::Scan { action }) => cli::scan::run(action).await,
        Some(Commands::Stats { action }) => cli::stats::run(action).await,
        Some(Commands::Serve { action }) => cli::serve::run(action).await,
        None => {
            let _ = App::command().print_help();
            print_newline!();
        }
    }
}
