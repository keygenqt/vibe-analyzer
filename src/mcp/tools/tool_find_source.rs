//! find_source tool handler.
//! Searches knowledge sources by name, license, or description.

use serde_json::Value;
use serde_json::json;

use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

/// Execute find_source tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Extract search query from arguments
    let query = match args.get("query").and_then(|q| q.as_str()) {
        Some(q) => q,
        None => return McpResponse::invalid_params(id),
    };
    // Extract limit parameter (default: 1)
    let limit = args.get("limit").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
    // Execute search and return results
    match state.search_client.search_meta(query, limit).await {
        Ok(results) => McpResponse::success(id, json!(results)),
        Err(e) => McpResponse::internal_error(id).with_data(json!({ "detail": e.to_string() })),
    }
}
