//! Response structure for the search_by_code_variables tool.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;

/// Response from search_by_code_variables tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SearchByCodeVariables")]
pub struct McpSearchByCodeVariablesResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching variables and enums.
    pub results: Vec<McpSearchByCodeVariablesMetaResponse>,
}

/// File metadata with matching variables and enums.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileMeta")]
pub struct McpSearchByCodeVariablesMetaResponse {
    /// Project root path.
    pub project: String,
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,
    /// Matching variable declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<AstDataVariable>,
    /// Matching enum declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<AstDataEnum>,
}

impl McpSearchByCodeVariablesResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} variable(s)/enum(s) found for '{}']\n\nUse this data to respond to the user.\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
