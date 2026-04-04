use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

use crate::configs::app::AppConfig;

mod cli;
mod configs;
mod mcp;
mod scanner;
mod utils;

#[derive(Parser)]
#[command(name = "vibe-analyzer")]
#[command(about=utils::app::app_about())]
#[command(styles=utils::app::app_styles())]
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
        #[arg(long)]
        path: Option<PathBuf>,
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
        // Scan and index project(s)
        Some(Commands::Scan { path }) => {
            if let Err(e) = cli::scan::exec(cli::scan::ScanOptions { custom_path: path }).await {
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
