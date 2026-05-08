//! JSON5 format exporter for source analysis results.
//! Provides human-readable JSON5 output with comments and trailing commas support.

use anyhow::Result;
use json5;
use std::fs;
use std::path::Path;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;

/// JSON5 export writer implementing the ExportWriter trait.
pub struct Json5Writer;

impl ExportWriter for Json5Writer {
    /// Exports source analyses to a human-readable JSON5 file.
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = json5::to_string(model)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
