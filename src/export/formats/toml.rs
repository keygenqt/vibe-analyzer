//! TOML format exporter for source analysis results.
//! Provides TOML output.

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;

/// TOML export writer implementing the ExportWriter trait.
pub struct TomlWriter;

impl ExportWriter for TomlWriter {
    /// Exports source analyses to a TOML file.
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = toml::to_string(model)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
