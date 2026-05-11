//! Response structure for the search_by_code_imports tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::file_models::FileModels;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;

/// Response from search_by_code_imports tool.
#[derive(Debug, Serialize)]
pub struct SearchByCodeImportsResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching imports.
    pub results: Vec<SearchByCodeImportsMetaResponse>,
}

/// File metadata with matching imports.
#[derive(Debug, Serialize)]
pub struct SearchByCodeImportsMetaResponse {
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    pub header_comments: Vec<String>,
    /// Matching import statements.
    pub imports: Vec<String>,
}

/// Converts file model to search result, filtering non-language files.
impl TryFrom<&FileModels> for SearchByCodeImportsMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchByCodeImportsMetaResponse {
                path: m.base_model().path().to_string(),
                language: m.language.clone(),
                header_comments: m.code.header_comments.clone(),
                imports: m.code.imports.clone(),
            }),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for SearchByCodeImportsResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} import(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
