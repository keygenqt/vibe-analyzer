//! search_ast tool handler.
//! Searches in code structure (functions, classes, imports).

use serde_json::Value;
use serde_json::json;

use crate::configs::app::IndexType;
use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;

/// Execute search_ast tool.
pub async fn call(id: Option<Value>, args: &Value, state: &AppState) -> McpResponse {
    // Extract search query from arguments
    let query = match args.get("query").and_then(|q| q.as_str()) {
        Some(q) => q,
        None => return McpResponse::invalid_params(id),
    };
    // Extract limit parameter (default: 10)
    let limit = args.get("limit").and_then(|l| l.as_u64()).unwrap_or(10) as usize;
    // Determine which indexes to query
    let indexes = match args.get("target").and_then(|t| t.as_str()) {
        Some(target) => {
            // Search in specific source
            match state.config.get_index(target, IndexType::Analysis) {
                Ok(idx) => vec![idx],
                Err(e) => {
                    return McpResponse::invalid_params(id)
                        .with_data(json!({ "detail": e.to_string() }));
                }
            }
        }
        None => {
            // Search in all sources
            state.config.get_indexes(IndexType::Analysis)
        }
    };
    // Execute search and return results
    match state.search_client.search_ast(&indexes, query, limit).await {
        Ok(results) => McpResponse::success(id, json!(results)),
        Err(e) => McpResponse::internal_error(id).with_data(json!({ "detail": e.to_string() })),
    }
}
