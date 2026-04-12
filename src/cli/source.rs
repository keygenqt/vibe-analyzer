//! Knowledge source management commands.
//! Provides add, remove, and list operations for configured sources.

use std::path::PathBuf;

use clap::Subcommand;
use colored::Colorize;

use crate::configs::app::AppConfig;
use crate::exit_error;
use crate::print_info;
use crate::print_success;

/// Source management actions.
#[derive(Subcommand)]
pub enum SourceAction {
    /// Add a new knowledge source.
    Add {
        /// Path to source
        path: PathBuf,
    },

    /// Remove a knowledge source.
    Remove {
        /// Target source
        target: String,
    },

    /// List all configured sources.
    List,
}

/// Execute source commands.
pub fn run(action: SourceAction) {
    match action {
        SourceAction::Add { path } => match AppConfig::load() {
            Ok(mut config) => {
                config.add_source(path);
                if let Err(e) = config.save() {
                    exit_error!("{}", e);
                }
                print_success!("Source added to config");
            }
            Err(e) => exit_error!("{}", e),
        },
        SourceAction::Remove { target } => match AppConfig::load() {
            Ok(mut config) => match config.remove_source(&target) {
                Ok(removed) => print_success!("Removed: {}", removed.display()),
                Err(e) => exit_error!("{}", e),
            },
            Err(e) => exit_error!("{}", e),
        },
        SourceAction::List => match AppConfig::load() {
            Ok(config) => {
                if config.sources.is_empty() {
                    print_info!("No sources configured");
                } else {
                    println!("{}", "Configured sources:".green().bold());
                    for source in config.sources.iter() {
                        println!("- {}", source.display().to_string().blue());
                    }
                }
            }
            Err(e) => exit_error!("{}", e),
        },
    }
}
