//! Tool for retrieving file AST data.
//! Handles full AST retrieval requests.

use std::convert::TryFrom;
use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::params::get_file_ast_params::GetFileAstParams;
use crate::mcp::responses::McpResponse;
use crate::mcp::responses::get_file_ast_response::GetFileAstResponse;
use crate::mcp::server::AppState;
use crate::models::file_language::FileLanguageModel;
use crate::utils;

/// Tool for retrieving file AST data.
#[derive(Clone)]
pub struct GetFileAstTool {
    state: Arc<AppState>,
}

impl GetFileAstTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the AST retrieval.
    pub async fn execute(&self, params: GetFileAstParams) -> Result<CallToolResult, ErrorData> {
        // Normalize path
        let path = utils::normalize::path_normalize(&self.state.config, &params.path)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Create tools client for files_meta indices
        let client = self
            .state
            .open_search_client
            .client_tool_files_meta(None)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        // Query file AST
        let file = client
            .get_file_ast(&path)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Convert to language model
        let ast = FileLanguageModel::try_from(file)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        // Format and return
        let response = GetFileAstResponse { ast };
        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
