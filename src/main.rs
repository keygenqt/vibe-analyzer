//! Vibe Analyzer - Main entry point.
//! Parses CLI arguments and dispatches to the appropriate subcommands (add, del, list, scan).

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::export::options::ExportFormat;

mod analyzer;
mod cli;
mod configs;
mod export;
mod mcp;
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
    /// Add project to config
    Add { path: PathBuf },

    /// Del project from config
    Del { path: PathBuf },

    /// List configured projects
    List,

    /// Analyze project codebase
    Scan {
        /// Path to scan (overrides config)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,

        /// Output file path (optional)
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },

    /// Scan and analyze project
    Analyze {
        /// Path to scan (overrides config)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,

        /// Output file path (optional)
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    match App::parse().command {
        // Add project to configuration
        Some(Commands::Add { path }) => match AppConfig::load() {
            Ok(mut config) => {
                config.add_project(path);
                if let Err(e) = config.save() {
                    exit_error!("{}", e);
                }
                print_success!("Project added to config");
            }
            Err(e) => exit_error!("{}", e),
        },

        // Delete project from configuration
        Some(Commands::Del { path }) => match AppConfig::load() {
            Ok(mut config) => match config.del_project(path) {
                Ok(removed) => print_success!("Removed: {}", removed.display()),
                Err(e) => exit_error!("{}", e),
            },
            Err(e) => exit_error!("{}", e),
        },

        // List all configured projects
        Some(Commands::List) => match AppConfig::load() {
            Ok(config) => {
                if config.projects.is_empty() {
                    print_info!("No projects configured");
                } else {
                    println!("{}", "Configured projects:".green().bold());
                    for project in config.projects.iter() {
                        println!("- {}", project.display().to_string().blue());
                    }
                }
            }
            Err(e) => exit_error!("{}", e),
        },

        // Scan and index project(s) - extracts AST and metadata
        Some(Commands::Scan {
            path,
            format,
            output,
        }) => {
            if let Err(e) = cli::scan::exec(cli::scan::ScanOptions {
                custom_path: path,
                format,
                output,
            })
            .await
            {
                exit_error!("{}", e);
            }
        }

        // Analyze project(s) with LLM enrichment - adds descriptions and search tags
        Some(Commands::Analyze {
            path,
            format,
            output,
        }) => {
            if let Err(e) = cli::analyze::exec(cli::analyze::AnalyzeOptions {
                custom_path: path,
                format,
                output,
            })
            .await
            {
                exit_error!("{}", e);
            }
        }

        // Show help when no command is provided
        None => {
            let _ = App::command().print_help();
            println!();
        }
    }
}
