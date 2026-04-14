//! MCP server implementation for AI assistant integration.
//! Provides HTTP endpoints for knowledge base search and retrieval.

use std::path::PathBuf;

use clap::Subcommand;

use crate::exit_error;
use crate::mcp;
use crate::print_warning;

/// MCP server management commands.
#[derive(Subcommand)]
pub enum ServeAction {
    /// Start MCP server.
    Start {
        /// Host address to bind (overrides config).
        #[arg(long)]
        host: Option<String>,

        /// Port to listen on (overrides config).
        #[arg(long)]
        port: Option<u16>,

        /// Working directory (defaults to current directory).
        #[arg(long)]
        workdir: Option<PathBuf>,
    },

    /// Stop running MCP server.
    Stop,

    /// Show MCP server status.
    Status,
}

/// Execute serve commands.
pub async fn run(action: ServeAction) {
    match action {
        ServeAction::Start {
            host,
            port,
            workdir,
        } => {
            if let Err(e) = mcp::server::run_server(host, port, workdir).await {
                exit_error!("Failed to start MCP server: {}", e);
            }
        }
        ServeAction::Stop => {
            print_warning!("serve stop is not yet implemented");
        }
        ServeAction::Status => {
            print_warning!("serve status is not yet implemented");
        }
    }
}
