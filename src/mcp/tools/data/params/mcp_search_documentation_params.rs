//! Request parameters for the search_documentation tool.
//! Defines the structure for documentation search requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the search_documentation tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "SearchDocumentation")]
pub struct McpSearchDocumentationParams {
    /// Search query (required).
    pub query: String,
    /// Maximum number of results (default: 3).
    #[serde(default = "McpSearchDocumentationParams::default_limit")]
    pub limit: Option<i32>,
}

impl McpSearchDocumentationParams {
    /// Default result limit.
    pub const fn default_limit() -> Option<i32> {
        Some(3)
    }
}
