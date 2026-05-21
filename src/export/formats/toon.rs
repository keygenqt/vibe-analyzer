//! TOON format exporter for source analysis results.
//! Provides token-efficient output optimized for LLM consumption.

use anyhow::Result;
use serde_toon;
use std::fs;
use std::path::Path;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;
use crate::models::file_models::FileModels;

/// TOON export writer implementing the ExportWriter trait.
pub struct ToonWriter;

impl ExportWriter for ToonWriter {
    /// Exports source analyses to a token-efficient TOON file.
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = serde_toon::to_string(model)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    /// Exports a single file model to a token-efficient TOON file.
    fn export_file(&self, source: &FileModels, output_path: &Path) -> Result<()> {
        let content = serde_toon::to_string(source)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
