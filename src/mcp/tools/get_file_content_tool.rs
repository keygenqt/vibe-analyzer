//! Tool for retrieving file content.
//! Handles full file content requests.

use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::configs::app::IndexType;
use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_get_file_content_params::McpGetFileContentParams;
use crate::mcp::tools::data::responses::mcp_get_file_content_response::McpGetFileContentResponse;
use crate::scanner::ast::parser::get_language_name;
use crate::utils::mcp_normalize::path_normalize;

/// Tool for retrieving file content.
#[derive(Clone)]
pub struct GetFileContentTool {
    state: Arc<AppState>,
}

impl GetFileContentTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the file retrieval.
    pub async fn execute(
        &self,
        params: McpGetFileContentParams,
    ) -> Result<CallToolResult, ErrorData> {
        let (root, path) = path_normalize(&self.state.config, &params.path)
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

        let indexes = match &root {
            Some(root) => {
                let index = self
                    .state
                    .config
                    .get_index(root, IndexType::Files)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                vec![index]
            }
            None => self.state.config.get_indexes(IndexType::Files),
        };

        let content = self
            .state
            .search_client
            .get_file_content(&indexes, root.as_deref(), &path)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        let ext = std::path::Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();

        let response = McpGetFileContentResponse {
            root,
            path: path.clone(),
            language: get_language_name(ext).unwrap_or("Unknown").to_string(),
            content,
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
