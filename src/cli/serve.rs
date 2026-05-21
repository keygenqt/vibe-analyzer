//! MCP server implementation for AI assistant integration.
//! Provides HTTP endpoints for knowledge base search and retrieval.

use std::path::PathBuf;

use clap::Subcommand;

use crate::exit_error;
use crate::mcp;
use crate::mcp::serve::ServeMcp;
use crate::print_info;
use crate::print_success;
use crate::print_warning;

/// MCP server management commands.
#[derive(Subcommand)]
pub enum ServeAction {
    /// Start MCP server.
    Start {
        /// Host address to bind.
        #[arg(long)]
        host: Option<String>,

        /// Port to listen on.
        #[arg(long)]
        port: Option<u16>,

        /// Working directory (defaults to current directory).
        #[arg(long)]
        workdir: Option<PathBuf>,

        /// MCP protocol version (2024-11-05, 2025-03-26, 2025-06-18, or "latest").
        #[arg(long)]
        protocol: Option<String>,
    },

    /// Stop running MCP server(s).
    Stop {
        /// Stop only the server on this port. If not specified, stops all.
        #[arg(long)]
        port: Option<u16>,
    },

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
            protocol,
        } => {
            if let Err(e) = mcp::server::run_server(
                host,
                port,
                workdir,
                protocol,
                |pid, host, port, protocol| {
                    let server = ServeMcp::new(pid, host, port, protocol);
                    server.save().ok();
                },
            )
            .await
            {
                exit_error!("Failed to start MCP server: {}", e);
            }
        }
        ServeAction::Stop { port } => {
            let servers = ServeMcp::read().unwrap_or_default();
            for server in &servers {
                if port.is_none() || Some(server.port) == port {
                    server.stop().ok();
                }
            }
            print_success!("MCP server(s) stopped");
        }
        ServeAction::Status => match ServeMcp::read() {
            Ok(servers) => {
                if servers.is_empty() {
                    print_info!("No MCP servers running");
                } else {
                    print_info!("MCP servers running:");
                    for s in servers {
                        print_info!(
                            "  PID: {}  Host: {}  Port: {}  Protocol: {}  Uptime: {}",
                            s.pid,
                            s.host,
                            s.port,
                            s.protocol,
                            s.load_time()
                        );
                    }
                }
            }
            Err(e) => print_warning!("Failed to read server stats: {}", e),
        },
    }
}
