//! Tool for retrieving file AST data.
//! Handles full AST retrieval requests.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_get_file_ast_params::McpGetFileAstParams;
use crate::mcp::tools::data::responses::mcp_get_file_ast_response::McpGetFileAstResponse;
use crate::utils::mcp_normalize::path_normalize;

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
    pub async fn execute(&self, params: McpGetFileAstParams) -> Result<CallToolResult, ErrorData> {
        let (root, path) = path_normalize(&self.state.config, &params.path)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        let indexes = match &root {
            Some(root) => {
                let index = self
                    .state
                    .config
                    .get_index(root, IndexType::Analysis)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                vec![index]
            }
            None => self.state.config.get_indexes(IndexType::Analysis),
        };

        let source = self
            .state
            .search_client
            .get_file_ast(&indexes, root.as_deref(), &path)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        let response = McpGetFileAstResponse::from(source);

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
