//! Request parameters for the search_by_docs tool.
//! Defines the structure for documentation search requests.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the search_by_docs tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "SearchByDocs")]
pub struct McpSearchByDocsParams {
    /// Search query.
    pub query: String,
    /// Maximum number of results.
    #[serde(default = "McpSearchByDocsParams::default_limit")]
    pub limit: Option<i32>,
}

impl McpSearchByDocsParams {
    /// Default result limit.
    pub const fn default_limit() -> Option<i32> {
        Some(3)
    }
}
