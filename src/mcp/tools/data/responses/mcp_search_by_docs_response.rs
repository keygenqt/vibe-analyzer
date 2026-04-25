//! Response structure for the search_by_docs tool.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::opensearch::data::responses::opensearch_file_meta_response::OpensearchFileMetaResponse;
use crate::scanner::ast::data::ast_data_heading::AstHeading;
use crate::scanner::ast::data::ast_data_links::AstLink;

/// Response from search_by_docs tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SearchByDocs")]
pub struct McpSearchByDocsResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching documentation.
    pub results: Vec<McpSearchByDocsMetaResponse>,
}

/// File metadata with matching documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FileMeta")]
pub struct McpSearchByDocsMetaResponse {
    /// Project root path.
    pub project: String,
    /// Relative file path.
    pub path: String,
    /// Markdown frontmatter metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<HashMap<String, String>>,
    /// Matching headings.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headings: Vec<AstHeading>,
    /// Matching links.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<AstLink>,
    /// Code block languages found.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code_blocks: Vec<String>,
}

impl From<OpensearchFileMetaResponse> for McpSearchByDocsMetaResponse {
    fn from(r: OpensearchFileMetaResponse) -> Self {
        let ast = r.ast.unwrap_or_default();
        Self {
            project: r.root,
            path: r.path,
            frontmatter: ast.frontmatter,
            headings: ast.headings,
            links: ast.links,
            code_blocks: ast.code_blocks,
        }
    }
}

impl McpSearchByDocsResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        format!(
            "[{} doc(s) found for '{}']\n\nUse this data to respond to the user.\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
