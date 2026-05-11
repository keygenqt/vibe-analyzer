//! Request parameters for the admin_sync tool.
//! Defines the structure for admin sync requests.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the admin_sync tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AdminSyncParams {
    /// Fast indexing, with reduced search quality (default: false).
    #[serde(default = "AdminSyncParams::default_fast")]
    pub fast: bool,
}

impl AdminSyncParams {
    /// Default fast mode.
    pub const fn default_fast() -> bool {
        false
    }
}
