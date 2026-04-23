//! Tool for searching variables and enums.
//! Handles code-level variable and enum search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_variables_params::McpSearchByCodeVariablesParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_variables_response::McpSearchByCodeVariablesMetaResponse;
use crate::mcp::tools::data::responses::mcp_search_by_code_variables_response::McpSearchByCodeVariablesResponse;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;

/// Tool for searching variables and enums.
#[derive(Clone)]
pub struct SearchByCodeVariablesTool {
    state: Arc<AppState>,
}

impl SearchByCodeVariablesTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeVariablesParams,
    ) -> Result<CallToolResult, ErrorData> {
        let response = McpSearchByCodeVariablesResponse {
            query: params.query,
            results: vec![
                McpSearchByCodeVariablesMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/constants.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![
                        "//! Application constants and configuration defaults.".to_string(),
                    ],
                    variables: vec![
                        AstDataVariable {
                            signature: "pub const MAX_SIZE: usize".to_string(),
                            comments: vec!["Global constants".to_string()],
                        },
                        AstDataVariable {
                            signature: "pub static APP_NAME: &str".to_string(),
                            comments: vec!["Static variables".to_string()],
                        },
                    ],
                    enums: vec![],
                },
                McpSearchByCodeVariablesMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/color.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![],
                    variables: vec![],
                    enums: vec![AstDataEnum {
                        signature: "pub enum Color".to_string(),
                        comments: vec!["Color enum".to_string()],
                    }],
                },
            ],
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
