//! Response structure for the get_file_ast tool.

use serde::{Deserialize, Serialize};

use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::parser::get_language_name;

/// Response from get_file_ast tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileAst")]
pub struct McpGetFileAstResponse {
    /// Project root path.
    pub root: String,
    /// File path.
    pub path: String,
    /// Programming language of the file.
    pub language: String,
    /// Full AST data.
    pub ast: AstData,
}

impl From<OpensearchFileMetaResponse> for McpGetFileAstResponse {
    fn from(r: OpensearchFileMetaResponse) -> Self {
        let ext = r.extension.as_deref().unwrap_or_default();
        Self {
            root: r.root,
            path: r.path,
            language: get_language_name(ext).unwrap_or("Unknown").to_string(),
            ast: r.ast.unwrap_or_default(),
        }
    }
}

impl McpGetFileAstResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[AST for {}]\n\nUse this data to respond to the user.\n{}",
            self.path,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
