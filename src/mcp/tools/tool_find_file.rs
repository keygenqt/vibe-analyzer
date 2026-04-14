//! find_file tool handler.
//! Searches files by path pattern in a project.

use serde_json::Value;
use serde_json::json;

use crate::configs::app::IndexType;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

/// Execute find_file tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Extract search pattern from arguments
    let pattern = match args.get("pattern").and_then(|p| p.as_str()) {
        Some(p) => p,
        None => return McpResponse::invalid_params(id),
    };
    // Extract target project name from arguments
    let target = match args.get("target").and_then(|t| t.as_str()) {
        Some(t) => t,
        None => return McpResponse::invalid_params(id),
    };
    // Extract limit parameter (default: 10)
    let limit = args.get("limit").and_then(|l| l.as_u64()).unwrap_or(10) as usize;
    // Get OpenSearch index for the target project
    let index = match state.config.get_index(target, IndexType::Files) {
        Ok(idx) => idx,
        Err(e) => {
            return McpResponse::invalid_params(id).with_data(json!({ "detail": e.to_string() }));
        }
    };
    // Execute search and return results
    match state.search_client.get_files(pattern, &index, limit).await {
        Ok(results) => McpResponse::success(id, json!(results)),
        Err(e) => McpResponse::internal_error(id).with_data(json!({ "detail": e.to_string() })),
    }
}
