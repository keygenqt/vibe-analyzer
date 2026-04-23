//! Response structure for the search_by_code_functions tool.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data_function::AstDataFunction;

/// Response from search_by_code_functions tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SearchByCodeFunctions")]
pub struct McpSearchByCodeFunctionsResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching functions.
    pub results: Vec<McpSearchByCodeFunctionsMetaResponse>,
}

/// File metadata with matching functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileMeta")]
pub struct McpSearchByCodeFunctionsMetaResponse {
    /// Project root path.
    pub project: String,
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,
    /// Matching function declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<AstDataFunction>,
}

impl McpSearchByCodeFunctionsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} file(s) found for '{}']\n\nUse this data to respond to the user.\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
