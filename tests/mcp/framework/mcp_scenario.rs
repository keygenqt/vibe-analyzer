//! Test scenario structures.
//! Defines scenario format for MCP tests.

use serde::Deserialize;
use serde::Serialize;

/// Test scenario loaded from JSON file.
#[derive(Debug, Serialize, Deserialize)]
pub struct McpScenario {
    /// MCP tool name to test.
    pub tool: String,
    /// Whether to clear context between queries.
    #[serde(default = "default_clear_context")]
    pub clear_context: bool,
    /// List of test queries.
    pub queries: Vec<String>,
}

fn default_clear_context() -> bool {
    true
}
