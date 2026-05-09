//! Export configuration.
//! Default format and output directory for scan exports.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::export::options::ExportFormat;

/// Export settings for scan ast and scan analyze commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Default export format (json, json5, toml, toon, xml, yaml).
    pub format: String,
    /// Default output directory.
    pub output_dir: PathBuf,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json.extension().to_string(),
            output_dir: dirs::download_dir().expect("Failed to get downloads directory"),
        }
    }
}
