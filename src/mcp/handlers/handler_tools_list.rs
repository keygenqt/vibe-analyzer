//! MCP tools/list handler.
//! Returns available tools with their input schemas.

use serde_json::Value;

use crate::mcp::api::mcp_response::McpResponse;

/// Handle tools/list request.
pub async fn handle(id: Option<Value>) -> McpResponse {
    McpResponse::success(
        id,
        serde_json::json!({
            "tools": [
                {
                    "name": "find_file",
                    "description": "Find file by path pattern in a project",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "pattern": { "type": "string", "description": "Path pattern to search" },
                            "target": { "type": "string", "description": "Target project name" },
                            "limit": { "type": "integer", "description": "Maximum results (default: 10)" }
                        },
                        "required": ["pattern", "target"]
                    }
                },
                {
                    "name": "find_source",
                    "description": "Find source project by name, license, or description",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": { "type": "string", "description": "Search query" },
                            "limit": { "type": "integer", "description": "Maximum results (default: 1)" }
                        },
                        "required": ["query"]
                    }
                },
                {
                    "name": "get_stats",
                    "description": "Get project statistics by language",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": { "type": "string", "description": "Target project name (all if not specified)" }
                        },
                        "required": []
                    }
                },
                {
                    "name": "get_tree",
                    "description": "Get directory tree structure",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": { "type": "string", "description": "Target project name" },
                            "level": { "type": "integer", "description": "Maximum depth (default: 3)" }
                        },
                        "required": ["target"]
                    }
                },
                {
                    "name": "index_source",
                    "description": "Scan project folder and update index",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": { "type": "string", "description": "Project name" }
                        },
                        "required": ["target"]
                    }
                },
                {
                    "name": "search_ast",
                    "description": "Search in code structure (functions, classes, imports)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": { "type": "string", "description": "Search query" },
                            "target": { "type": "string", "description": "Target project name (optional)" },
                            "limit": { "type": "integer", "description": "Maximum results (default: 10)" }
                        },
                        "required": ["query"]
                    }
                }
            ]
        }),
    )
}
