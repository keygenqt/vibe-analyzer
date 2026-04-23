//! Tool for searching classes, structs, and interfaces.
//! Handles code-level type search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_classes_params::McpSearchByCodeClassesParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_classes_response::McpSearchByCodeClassesMetaResponse;
use crate::mcp::tools::data::responses::mcp_search_by_code_classes_response::McpSearchByCodeClassesResponse;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;

/// Tool for searching classes, structs, and interfaces.
#[derive(Clone)]
pub struct SearchByCodeClassesTool {
    state: Arc<AppState>,
}

impl SearchByCodeClassesTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeClassesParams,
    ) -> Result<CallToolResult, ErrorData> {
        let response = McpSearchByCodeClassesResponse {
            query: params.query,
            results: vec![
                McpSearchByCodeClassesMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/models.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec!["//! Data models for the application".to_string()],
                    classes: vec![AstDataClass {
                        signature: "pub struct User".to_string(),
                        comments: vec!["User data model".to_string()],
                    }],
                    structs: vec![],
                    interfaces: vec![],
                },
                McpSearchByCodeClassesMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/services.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![],
                    classes: vec![],
                    structs: vec![],
                    interfaces: vec![AstDataInterface {
                        signature: "pub trait Repository".to_string(),
                        comments: vec!["Data access trait".to_string()],
                    }],
                },
            ],
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
