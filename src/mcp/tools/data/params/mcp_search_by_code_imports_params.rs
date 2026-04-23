//! @todo
//! @todo

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// @todo
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
