//! License detection result structure

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    /// SPDX identifier (e.g., "MIT", "Apache-2.0")
    pub spdx_id: String,
    /// License name (e.g., "MIT License")
    pub name: String,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Path to license file
    pub path: String,
}
