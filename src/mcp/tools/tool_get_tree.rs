//! get_tree tool handler.
//! Returns directory tree structure for a source.

use serde_json::Value;
use serde_json::json;

use crate::configs::app::IndexType;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

/// Execute get_tree tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Extract target source name from arguments
    let target = match args.get("target").and_then(|t| t.as_str()) {
        Some(t) => t,
        None => return McpResponse::invalid_params(id),
    };
    // Extract level parameter (default: 3)
    let level = args.get("level").and_then(|l| l.as_u64()).unwrap_or(3) as usize;
    // Get OpenSearch index for the target source
    let index = match state.config.get_index(target, IndexType::Files) {
        Ok(idx) => idx,
        Err(e) => {
            return McpResponse::invalid_params(id).with_data(json!({ "detail": e.to_string() }));
        }
    };
    // Execute tree query and return results
    match state.search_client.get_tree(&index, level).await {
        Ok(result) => McpResponse::success(id, json!(result)),
        Err(e) => McpResponse::internal_error(id).with_data(json!({ "detail": e.to_string() })),
    }
}
