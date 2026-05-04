//! Response structure for the search_by_code_variables tool.

use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::{
    mcp::responses::McpResponse,
    models::{
        ast_enum::AstEnumModel, ast_model::AstModel, ast_variable::AstVariableModel,
        file_models::FileModels,
    },
};

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
    pub variables: Vec<AstVariableModel>,
    /// Matching enum declarations.
    pub enums: Vec<AstEnumModel>,
}

/// Converts file model to search result, filtering non-language files.
impl TryFrom<&FileModels> for SearchByCodeVariablesMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchByCodeVariablesMetaResponse {
                path: m.path.clone(),
                language: m.language.clone(),
                header_comments: m.header_comments.clone(),
                variables: m.variables.iter().map(|v| v.format_tool()).collect(),
                enums: m.enums.iter().map(|e| e.format_tool()).collect(),
            }),
            _ => anyhow::bail!("AST not available for this file type"),
        }
    }
}

impl McpResponse for SearchByCodeVariablesResponse {
    fn format(&self) -> String {
        format!(
            "[{} variable(s)/enum(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
