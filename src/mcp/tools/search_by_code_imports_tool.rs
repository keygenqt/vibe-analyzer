//! Tool for searching import statements.
//! Handles code-level import queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_imports_params::McpSearchByCodeImportsParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_imports_response::McpSearchByCodeImportsMetaResponse;
use crate::mcp::tools::data::responses::mcp_search_by_code_imports_response::McpSearchByCodeImportsResponse;

/// Tool for searching import statements.
#[derive(Clone)]
pub struct SearchByCodeImportsTool {
    state: Arc<AppState>,
}

impl SearchByCodeImportsTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeImportsParams,
    ) -> Result<CallToolResult, ErrorData> {
        let response = McpSearchByCodeImportsResponse {
            query: params.query,
            results: vec![
                McpSearchByCodeImportsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/main.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec!["//! Main entry point".to_string()],
                    imports: vec![
                        "std::collections::HashMap".to_string(),
                        "serde::Deserialize".to_string(),
                    ],
                },
                McpSearchByCodeImportsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/lib.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![],
                    imports: vec!["tokio::main".to_string(), "serde::Serialize".to_string()],
                },
            ],
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
