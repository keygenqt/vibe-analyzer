//! Request parameters for the search_by_code_classes tool.
//! Defines the structure for code classes, structs, and interfaces search requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the search_by_code_classes tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "SearchByCodeClasses")]
pub struct McpSearchByCodeClassesParams {
    /// Search query.
    pub query: String,
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum number of results.
    #[serde(default = "McpSearchByCodeClassesParams::default_limit")]
    pub limit: Option<i32>,
}

impl McpSearchByCodeClassesParams {
    /// Default result limit.
    pub const fn default_limit() -> Option<i32> {
        Some(3)
    }
}
