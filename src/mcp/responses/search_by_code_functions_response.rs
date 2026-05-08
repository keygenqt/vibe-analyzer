//! Response structure for the search_by_code_functions tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::ast_function::AstFunctionModel;
use crate::models::ast_model::AstModel;
use crate::models::file_models::FileModels;

/// Response from search_by_code_functions tool.
#[derive(Debug, Serialize)]
pub struct SearchByCodeFunctionsResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching functions.
    pub results: Vec<SearchByCodeFunctionsMetaResponse>,
}

/// File metadata with matching functions.
#[derive(Debug, Serialize)]
pub struct SearchByCodeFunctionsMetaResponse {
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    pub header_comments: Vec<String>,
    /// Matching function declarations.
    pub functions: Vec<AstFunctionModel>,
}

/// Converts file model to search result, filtering non-language files.
impl TryFrom<&FileModels> for SearchByCodeFunctionsMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchByCodeFunctionsMetaResponse {
                path: m.path.clone(),
                language: m.language.clone(),
                header_comments: m.header_comments.clone(),
                functions: m.functions.iter().map(|f| f.format_tool()).collect(),
            }),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for SearchByCodeFunctionsResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} file(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
