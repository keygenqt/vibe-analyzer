//! MCP server information structures.
//! Provides server metadata for MCP client initialization.

use serde::Serialize;

use crate::utils::app;

/// Server information returned on initialization.
#[derive(Serialize)]
pub struct McpInfo {
    /// Status code (always 200).
    pub code: u16,
    /// Server version from Cargo.toml.
    pub version: String,
}

impl Default for McpInfo {
    fn default() -> Self {
        Self {
            code: 200,
            version: app::app_version().to_string(),
        }
    }
}
