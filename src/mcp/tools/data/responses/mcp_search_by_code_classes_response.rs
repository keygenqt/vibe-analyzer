//! Response structure for the search_by_code_classes tool.

use serde::Deserialize;
use serde::Serialize;

use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;
use crate::scanner::ast::parser::get_language_name;

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

impl From<OpensearchFileMetaResponse> for McpSearchByCodeClassesMetaResponse {
    fn from(r: OpensearchFileMetaResponse) -> Self {
        let ext = r.extension.as_deref().unwrap_or_default();
        let ast = r.ast.unwrap_or_default();
        Self {
            project: r.root,
            path: r.path,
            language: get_language_name(ext).unwrap_or("Unknown").to_string(),
            header_comments: ast.header_comments,
            classes: ast.classes,
            structs: ast.structs,
            interfaces: ast.interfaces,
        }
    }
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
