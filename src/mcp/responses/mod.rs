//! Response models for MCP tools.
//! Each tool has its own response structure.

pub mod admin_sync_response;
pub mod get_file_ast_response;
pub mod get_file_content_response;
pub mod search_by_code_classes_response;
pub mod search_by_code_functions_response;
pub mod search_by_code_imports_response;
pub mod search_by_code_variables_response;
pub mod search_documentation_response;
pub mod show_projects_response;
pub mod show_stats_response;
pub mod show_tree_response;

/// Common interface for MCP tool responses.
pub trait McpResponse {
    /// Returns formatted text for AI model consumption.
    fn format(&self) -> String;
}
