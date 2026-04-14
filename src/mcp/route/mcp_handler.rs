// src/mcp/route/mcp_handler.rs

//! Main MCP endpoint dispatcher.
//! Routes JSON-RPC requests to existing handlers.

use axum::Json;
use axum::extract::State;
use serde_json::Value;

use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::handlers::handler_initialize;
use crate::mcp::handlers::handler_tools_call;
use crate::mcp::handlers::handler_tools_list;
use crate::mcp::server::AppState;

/// POST /mcp
pub async fn post(State(state): State<AppState>, Json(payload): Json<Value>) -> McpResponse {
    let id = payload.get("id").cloned();
    let method = payload.get("method").and_then(|m| m.as_str()).unwrap_or("");
    match method {
        "initialize" => handler_initialize::handle(id).await,
        "tools/list" => handler_tools_list::handle(id).await,
        "tools/call" => {
            let params = match payload.get("params") {
                Some(p) => p,
                None => return McpResponse::invalid_params(id),
            };
            let name = match params.get("name").and_then(|n| n.as_str()) {
                Some(n) => n.to_string(),
                None => return McpResponse::invalid_params(id),
            };
            let args = params.get("arguments").cloned().unwrap_or(Value::Null);
            handler_tools_call::handle(id, name, args, state).await
        }
        _ => McpResponse::method_not_found(id),
    }
}
