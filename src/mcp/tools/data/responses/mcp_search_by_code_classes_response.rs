//! Response structure for the search_by_code_classes tool.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;

/// Response from search_by_code_classes tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SearchByCodeClasses")]
pub struct McpSearchByCodeClassesResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching classes, structs, or interfaces.
    pub results: Vec<McpSearchByCodeClassesMetaResponse>,
}

/// File metadata with matching classes, structs, and interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileMeta")]
pub struct McpSearchByCodeClassesMetaResponse {
    /// Project root path.
    pub project: String,
    /// Relative file path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,
    /// Matching class declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<AstDataClass>,
    /// Matching struct declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<AstDataStruct>,
    /// Matching interface declarations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<AstDataInterface>,
}

impl McpSearchByCodeClassesResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} type(s) found for '{}']\n\nUse this data to respond to the user.\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
