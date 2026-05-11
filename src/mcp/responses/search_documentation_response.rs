//! Response structure for the search_documentation tool.

use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::Result;
use serde::Serialize;

use crate::mcp::responses::McpResponse;
use crate::models::file_models::FileModels;
use crate::models::lang_heading::LangHeadingModel;
use crate::models::lang_link::LangLinkModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;

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
    pub headings: Vec<LangHeadingModel>,
    /// Matching links.
    pub links: Vec<LangLinkModel>,
    /// Code block languages found.
    pub code_blocks: Vec<String>,
}

/// Converts file model to search result.
impl TryFrom<&FileModels> for SearchDocumentationMetaResponse {
    type Error = anyhow::Error;

    fn try_from(model: &FileModels) -> Result<Self> {
        match model {
            FileModels::Language(m) => Ok(SearchDocumentationMetaResponse {
                path: m.base_model().path().to_string(),
                frontmatter: m.knowledge.frontmatter.clone(),
                headings: m
                    .knowledge
                    .headings
                    .iter()
                    .map(|h| h.format_tool())
                    .collect(),
                links: m.knowledge.links.iter().map(|l| l.format_tool()).collect(),
                code_blocks: m.knowledge.code_blocks.clone(),
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
