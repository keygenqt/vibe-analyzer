//! Request parameters for the show_projects tool.
//! Provides optional limit for project listing.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the show_projects tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShowProjectsParams {
    /// Maximum size (default: 3).
    #[serde(default = "ShowProjectsParams::default_size")]
    pub size: Option<i32>,
}

impl ShowProjectsParams {
    /// Default result limit.
    pub const fn default_size() -> Option<i32> {
        Some(3)
    }
}
