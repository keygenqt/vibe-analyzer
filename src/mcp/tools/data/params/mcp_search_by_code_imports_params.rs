//! Request parameters for the search_by_code_imports tool.
//! Defines the structure for code import search requests

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the search_by_code_imports tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "SearchByCodeImports")]
pub struct McpSearchByCodeImportsParams {
    /// Search query.
    pub query: String,
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum number of results.
    #[serde(default = "McpSearchByCodeImportsParams::default_limit")]
    pub limit: Option<i32>,
}

impl McpSearchByCodeImportsParams {
    /// Default result limit.
    pub const fn default_limit() -> Option<i32> {
        Some(3)
    }
}
