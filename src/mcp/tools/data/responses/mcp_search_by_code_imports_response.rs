//! Response structure for the search_by_code_imports tool.

use serde::Deserialize;
use serde::Serialize;

/// Response from search_by_code_imports tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SearchByCodeImports")]
pub struct McpSearchByCodeImportsResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching imports.
    pub results: Vec<McpSearchByCodeImportsMetaResponse>,
}

/// File metadata with matching imports.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileMeta")]
pub struct McpSearchByCodeImportsMetaResponse {
    /// Project root path.
    pub project: String,
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,
    /// Matching import statements.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,
}

impl McpSearchByCodeImportsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} import(s) found for '{}']\n\nUse this data to respond to the user.\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
