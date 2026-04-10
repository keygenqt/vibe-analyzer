//! MCP server implementation for AI assistant integration.
//! Provides HTTP endpoints for knowledge base search and retrieval.

use std::path::PathBuf;

use clap::Subcommand;

use crate::print_warning;

/// MCP server management commands.
#[derive(Subcommand)]
pub enum ServeAction {
    /// Start MCP server on specified host and port.
    Start {
        /// Host address to bind
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// Port number to listen on
        #[arg(long, default_value = "3000")]
        port: i32,

        /// Working directory (defaults to current directory)
        #[arg(long)]
        workdir: Option<PathBuf>,
    },

    /// Stop running MCP server gracefully.
    Stop,

    /// Show MCP server status and health information.
    Status,
}

/// Execute serve commands.
#[allow(unused_variables)]
pub fn run(action: ServeAction) {
    match action {
        ServeAction::Start {
            host,
            port,
            workdir,
        } => {
            print_warning!("Coming soon: serve start");
        }
        ServeAction::Stop => {
            print_warning!("Coming soon: serve stop");
        }
        ServeAction::Status => {
            print_warning!("Coming soon: serve status");
        }
    }
}
