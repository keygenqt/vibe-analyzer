//! get_stats tool handler.
//! Returns project statistics by language.

use serde_json::Value;
use serde_json::json;

use crate::configs::app::IndexType;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

/// Execute get_stats tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Determine which indexes to query
    let indexes = match args.get("target").and_then(|t| t.as_str()) {
        Some(target) => {
            // Get stats for specific source
            match state.config.get_index(target, IndexType::Analysis) {
                Ok(idx) => vec![idx],
                Err(e) => {
                    return McpResponse::invalid_params(id)
                        .with_data(json!({ "detail": e.to_string() }));
                }
            }
        }
        None => {
            // Get stats for all sources
            state.config.get_indexes(IndexType::Analysis)
        }
    };
    // Execute stats query and return results
    match state.search_client.get_stats(&indexes).await {
        Ok(results) => McpResponse::success(id, json!(results)),
        Err(e) => McpResponse::internal_error(id).with_data(json!({ "detail": e.to_string() })),
    }
}
