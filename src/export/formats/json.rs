//! JSON format exporter for source analysis results.
//! Provides compact JSON output with minimal size (no extra whitespace).

use anyhow::Result;
use serde_json;
use std::fs;
use std::path::Path;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;
use crate::models::file_models::FileModels;

/// JSON export writer implementing the ExportWriter trait.
pub struct JsonWriter;

impl ExportWriter for JsonWriter {
    /// Exports source analyses to a compact JSON file (minified).
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = serde_json::to_string(&model)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    /// Exports a single file model to a compact JSON file.
    fn export_file(&self, source: &FileModels, output_path: &Path) -> Result<()> {
        let content = serde_json::to_string(&source)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
