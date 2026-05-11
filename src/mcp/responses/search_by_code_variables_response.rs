//! Response structure for the search_by_code_variables tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::file_models::FileModels;
use crate::models::lang_enum::LangEnumModel;
use crate::models::lang_variable::LangVariableModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel as _;
use crate::models::trait_format_model::TraitFormatModel;

/// Response from search_by_code_variables tool.
#[derive(Debug, Serialize)]
pub struct SearchByCodeVariablesResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching variables and enums.
    pub results: Vec<SearchByCodeVariablesMetaResponse>,
}

/// File metadata with matching variables and enums.
#[derive(Debug, Serialize)]
pub struct SearchByCodeVariablesMetaResponse {
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    pub header_comments: Vec<String>,
    /// Matching variable declarations.
    pub variables: Vec<LangVariableModel>,
    /// Matching enum declarations.
    pub enums: Vec<LangEnumModel>,
}

/// Converts file model to search result, filtering non-language files.
impl TryFrom<&FileModels> for SearchByCodeVariablesMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchByCodeVariablesMetaResponse {
                path: m.base_model().path().to_string(),
                language: m.language.clone(),
                header_comments: m.code.header_comments.clone(),
                variables: m.code.variables.iter().map(|v| v.format_tool()).collect(),
                enums: m.code.enums.iter().map(|e| e.format_tool()).collect(),
            }),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for SearchByCodeVariablesResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} variable(s)/enum(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
