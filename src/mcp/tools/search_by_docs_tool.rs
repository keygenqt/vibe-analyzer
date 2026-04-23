//! Tool for searching documentation.
//! Handles docs search queries.

use std::collections::HashMap;
use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use crate::mcp::server::AppState;
use crate::mcp::tools::data::params::mcp_search_by_docs_params::McpSearchByDocsParams;
use crate::mcp::tools::data::responses::mcp_search_by_docs_response::{
    McpSearchByDocsMetaResponse, McpSearchByDocsResponse,
};
use crate::scanner::ast::data::ast_data_heading::AstHeading;
use crate::scanner::ast::data::ast_data_links::AstLink;

/// Tool for searching documentation.
#[derive(Clone)]
pub struct SearchByDocsTool {
    state: Arc<AppState>,
}

impl SearchByDocsTool {
    /// Creates a new instance.
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Executes the search.
    pub async fn execute(
        &self,
        params: McpSearchByDocsParams,
    ) -> Result<CallToolResult, ErrorData> {
        let mut frontmatter = HashMap::new();
        frontmatter.insert(
            "title".to_string(),
            "Vibe Analyzer Documentation".to_string(),
        );
        frontmatter.insert("author".to_string(), "keygenqt".to_string());

        let response = McpSearchByDocsResponse {
            query: params.query,
            results: vec![
                McpSearchByDocsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "docs/guide.md".to_string(),
                    frontmatter: Some(frontmatter.clone()),
                    headings: vec![
                        AstHeading {
                            level: 1,
                            title: "Getting Started".to_string(),
                            preview: "How to install and configure Vibe Analyzer".to_string(),
                        },
                        AstHeading {
                            level: 2,
                            title: "Installation".to_string(),
                            preview: "Install via cargo or download binary".to_string(),
                        },
                    ],
                    links: vec![AstLink {
                        text: "Rust guideline".to_string(),
                        url: "code/rust.md".to_string(),
                    }],
                    code_blocks: vec!["bash".to_string(), "rust".to_string()],
                },
                McpSearchByDocsMetaResponse {
                    project: "/path/to/project".to_string(),
                    path: "docs/api.md".to_string(),
                    frontmatter: Some(frontmatter),
                    headings: vec![AstHeading {
                        level: 1,
                        title: "API Reference".to_string(),
                        preview: "Complete API documentation".to_string(),
                    }],
                    links: vec![],
                    code_blocks: vec!["json".to_string()],
                },
            ],
        };

        Ok(CallToolResult::success(vec![Content::text(
            response.format(),
        )]))
    }
}
