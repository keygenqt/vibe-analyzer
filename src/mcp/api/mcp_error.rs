//! MCP JSON-RPC error structures.
//! Provides standard error codes and formatting for MCP responses.

use serde::Serialize;
use serde_json::Value;

/// MCP JSON-RPC error object.
#[derive(Serialize, Debug, Clone)]
pub struct McpError {
    /// Error code (standard JSON-RPC or custom).
    pub code: i32,
    /// Human-readable error message.
    pub message: String,
    /// Optional additional error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
