//! JSON format exporter for source analysis results.
//! Provides compact JSON output with minimal size (no extra whitespace).

use anyhow::Result;
use serde_json;
use std::fs;
use std::path::Path;

use crate::export::writer::{ExportModel, ExportWriter};

/// JSON export writer implementing the ExportWriter trait.
pub struct JsonWriter;

impl ExportWriter for JsonWriter {
    /// Exports source analyses to a compact JSON file (minified).
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = serde_json::to_string(&model)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
