//! MCP tools/call handler.
//! Dispatches tool execution to appropriate implementations.

use serde_json::Value;

use crate::mcp::api::mcp_response::McpResponse;
use crate::mcp::server::AppState;
use crate::mcp::tools::search_ast;
use crate::mcp::tools::tool_find_file;
use crate::mcp::tools::tool_find_source;
use crate::mcp::tools::tool_get_stats;
use crate::mcp::tools::tool_get_tree;
use crate::mcp::tools::tool_index_source;

/// Handle tools/call request.
pub async fn handle(id: Option<Value>, name: String, args: Value, state: AppState) -> McpResponse {
    match name.as_str() {
        "find_file" => tool_find_file::call(id, &args, &state).await,
        "find_source" => tool_find_source::call(id, &args, &state).await,
        "get_stats" => tool_get_stats::call(id, &args, &state).await,
        "get_tree" => tool_get_tree::call(id, &args, &state).await,
        "index_source" => tool_index_source::call(id, &args, &state).await,
        "search_ast" => search_ast::call(id, &args, &state).await,
        _ => McpResponse::method_not_found(id),
    }
}
