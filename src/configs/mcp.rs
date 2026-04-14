// src/configs/mcp.rs

//! MCP server configuration for AI assistant integration.
//! Defines server binding parameters and optional authentication.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for MCP HTTP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Server bind address (0.0.0.0 for all interfaces, 127.0.0.1 for local only).
    pub host: String,
    /// Server port (default: 9020).
    pub port: u16,
    /// API key for authentication (empty = no auth).
    pub api_key: String,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 9020,
            api_key: String::new(),
        }
    }
}
