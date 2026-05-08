//! Response structure for the search_documentation tool.

use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::ast_heading::AstHeadingModel;
use crate::models::ast_link::AstLinkModel;
use crate::models::ast_model::AstModel;
use crate::models::file_models::FileModels;

/// Response from search_documentation tool.
#[derive(Debug, Serialize)]
pub struct SearchDocumentationResponse {
    /// Search query that was used.
    pub query: String,
    /// Found files with matching documentation.
    pub results: Vec<SearchDocumentationMetaResponse>,
}

/// File metadata with matching documentation.
#[derive(Debug, Serialize)]
pub struct SearchDocumentationMetaResponse {
    /// Relative file path.
    pub path: String,
    /// Markdown frontmatter metadata.
    pub frontmatter: Option<HashMap<String, String>>,
    /// Matching headings.
    pub headings: Vec<AstHeadingModel>,
    /// Matching links.
    pub links: Vec<AstLinkModel>,
    /// Code block languages found.
    pub code_blocks: Vec<String>,
}

/// Converts file model to search result.
impl TryFrom<&FileModels> for SearchDocumentationMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchDocumentationMetaResponse {
                path: m.path.clone(),
                frontmatter: m.frontmatter.clone(),
                headings: m.headings.iter().map(|h| h.format_tool()).collect(),
                links: m.links.iter().map(|l| l.format_tool()).collect(),
                code_blocks: m.code_blocks.clone(),
            }),
            _ => anyhow::bail!("Not a documentation file"),
        }
    }
}

impl McpResponse for SearchDocumentationResponse {
    /// Returns a formatted tool result with JSON data.
    fn format(&self) -> String {
        format!(
            "[{} doc(s) found for '{}']\nUse this data to respond to the user.\n\n{}",
            self.results.len(),
            self.query,
            serde_json::to_string(self).unwrap_or_default()
        )
    }
}
