//! JSON format exporter for project analysis results.
//! Provides compact JSON output with minimal size (no extra whitespace).

use anyhow::Result;
use chrono::Local;
use serde_json;
use std::fs;
use std::path::Path;

use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::scanner::data::output::projects_analysis::ProjectsAnalysis;

use super::super::writer::ExportWriter;

/// JSON export writer implementing the ExportWriter trait.
pub struct JsonWriter;

impl ExportWriter for JsonWriter {
    /// Exports project analyses to a compact JSON file (minified).
    fn export(&self, projects: &[ProjectAnalysis], output_path: &Path) -> Result<()> {
        // Wrap projects with timestamp metadata
        let multi_analysis = ProjectsAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects.to_vec(),
        };

        // Serialize without indentation for minimal size
        let json_content = serde_json::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json_content)?;

        Ok(())
    }
}
