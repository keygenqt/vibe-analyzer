//! MCP JSON-RPC response structures.
//! Provides unified response format for all MCP method handlers.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use serde_json::Value;

use crate::mcp::api::mcp_error::McpError;

/// MCP JSON-RPC response.
#[derive(Serialize, Debug, Clone)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

/// Impl axum response for McpResponse.
impl IntoResponse for McpResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, axum::Json(self)).into_response()
    }
}

impl McpResponse {
    /// Create success response.
    pub fn success(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create error response.
    pub fn error(id: Option<Value>, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }

    /// Attach additional data to an existing error response.
    pub fn with_data(mut self, data: Value) -> Self {
        if let Some(ref mut err) = self.error {
            err.data = Some(data);
        }
        self
    }

    /// Server error (-32000 range reserved for implementation-defined server errors).
    pub fn server_error(id: Option<Value>, message: &str) -> Self {
        Self::error(id, -32000, message)
    }

    /// Method not found (-32601).
    pub fn method_not_found(id: Option<Value>) -> Self {
        Self::error(id, -32601, "Method not found")
    }

    /// Invalid params (-32602).
    pub fn invalid_params(id: Option<Value>) -> Self {
        Self::error(id, -32602, "Invalid params")
    }

    /// Internal error (-32603).
    pub fn internal_error(id: Option<Value>) -> Self {
        Self::error(id, -32603, "Internal error")
    }
}
