//! Tool for searching functions and methods.
//! Handles code-level function search queries.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_code_functions_params::McpSearchByCodeFunctionsParams;
use crate::mcp::tools::data::responses::mcp_search_by_code_functions_response::McpSearchByCodeFunctionsMetaResponse;
use crate::mcp::tools::data::responses::mcp_search_by_code_functions_response::McpSearchByCodeFunctionsResponse;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;

/// Tool for searching functions and methods.
#[derive(Clone)]
pub struct SearchByCodeFunctionsTool {
    state: Arc<AppState>,
}

impl SearchByCodeFunctionsTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByCodeFunctionsParams,
    ) -> Result<CallToolResult, ErrorData> {
        let response = McpSearchByCodeFunctionsResponse {
            query: params.query,
            results: vec![
                McpSearchByCodeFunctionsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/lib.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![
                        "//! Simple math library with error handling".to_string(),
                    ],
                    functions: vec![
                        AstDataFunction {
                            signature: "pub fn add(a: i32, b: i32) -> i32".to_string(),
                            comments: vec!["Calculates sum of two numbers".to_string()],
                        },
                        AstDataFunction {
                            signature: "pub fn multiply(a: i32, b: i32) -> i32".to_string(),
                            comments: vec!["Calculates product of two numbers".to_string()],
                        },
                    ],
                },
                McpSearchByCodeFunctionsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "src/main.rs".to_string(),
                    language: "rust".to_string(),
                    header_comments: vec![],
                    functions: vec![AstDataFunction {
                        signature: "fn main()".to_string(),
                        comments: vec![],
                    }],
                },
            ],
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
