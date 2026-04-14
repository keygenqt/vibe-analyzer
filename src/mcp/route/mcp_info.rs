use crate::mcp::api::mcp_json::McpJson;
use crate::mcp::models::mcp_info::McpInfo;

/// GET /
pub async fn get() -> McpJson<McpInfo> {
    McpJson(McpInfo::default())
}
