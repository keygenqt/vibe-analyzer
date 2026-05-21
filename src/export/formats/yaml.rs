//! YAML format exporter for source analysis results.
//! Provides human-readable YAML output.

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;
use crate::models::file_models::FileModels;

/// YAML export writer implementing the ExportWriter trait.
pub struct YamlWriter;

impl ExportWriter for YamlWriter {
    /// Exports source analyses to a YAML file.
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let content = yaml_serde::to_string(model)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    /// Exports a single file model to a YAML file.
    fn export_file(&self, source: &FileModels, output_path: &Path) -> Result<()> {
        let content = yaml_serde::to_string(source)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
