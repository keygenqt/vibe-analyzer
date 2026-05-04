//! Request parameters for the search_by_code_variables tool.
//! Defines the structure for code variables and enums search requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the search_by_code_variables tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchByCodeVariablesParams {
    /// Search query.
    pub query: String,
    /// Target project (optional) - project name.
    #[serde(default)]
    pub target: Option<String>,
    /// Maximum number of results.
    #[serde(default = "SearchByCodeVariablesParams::default_limit")]
    pub limit: Option<i32>,
}

impl SearchByCodeVariablesParams {
    /// Default result limit.
    pub const fn default_limit() -> Option<i32> {
        Some(3)
    }
}
