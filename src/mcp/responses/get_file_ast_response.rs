//! Response structure for the get_file_ast tool.
use std::convert::TryFrom;

use anyhow::Result;

use crate::mcp::responses::McpResponse;
use crate::models::file_language::FileLanguageModel;
use crate::models::file_model::FileModel;
use crate::models::file_models::FileModels;

/// Response from get_file_ast tool.
#[derive(Debug)]
pub struct GetFileAstResponse {
    pub ast: FileLanguageModel,
}

/// Converts file model to language model for AST retrieval.
impl TryFrom<FileModels> for FileLanguageModel {
    type Error = anyhow::Error;

    fn try_from(model: FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(m),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for GetFileAstResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        let mut json = serde_json::to_value(&self.ast.format_tool()).unwrap_or_default();
        // format_tool returns FileModels with serde tag "type", but we want flat AST without it
        json.as_object_mut().map(|obj| obj.remove("type"));
        format!(
            "[AST]\nUse this data to respond to the user.\n\n{}",
            serde_json::to_string(&json).unwrap_or_default()
        )
    }
}
