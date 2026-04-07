//! JSON5 format exporter for project analysis results.
//! Provides human-readable JSON5 output with comments and trailing commas support.

use anyhow::Result;
use chrono::Local;
use json5;
use std::fs;
use std::path::Path;

use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::scanner::data::output::projects_analysis::ProjectsAnalysis;

use super::super::writer::ExportWriter;

/// JSON5 export writer implementing the ExportWriter trait.
pub struct Json5Writer;

impl ExportWriter for Json5Writer {
    /// Exports project analyses to a human-readable JSON5 file.
    fn export(&self, projects: &[ProjectAnalysis], output_path: &Path) -> Result<()> {
        // Wrap projects with timestamp metadata
        let multi_analysis = ProjectsAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects.to_vec(),
        };

        // Serialize with pretty formatting (json5 always produces readable output)
        let json5_content = json5::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json5_content)?;

        Ok(())
    }
}
