//! MCP initialize handler.
//! Returns server capabilities and protocol version.

use serde_json::Value;

use crate::mcp::api::mcp_response::McpResponse;
use crate::utils::app;

/// Handle initialize request.
pub async fn handle(id: Option<Value>) -> McpResponse {
    McpResponse::success(
        id,
        serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": app::app_name(),
                "version": app::app_version()
            }
        }),
    )
}
