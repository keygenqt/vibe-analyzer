//! TOON format exporter for project analysis results.
//! Provides token-efficient output optimized for LLM consumption.

use anyhow::Result;
use chrono::Local;
use serde_toon;
use std::fs;
use std::path::Path;

use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::scanner::data::output::projects_analysis::ProjectsAnalysis;

use super::super::writer::ExportWriter;

/// TOON export writer implementing the ExportWriter trait.
pub struct ToonWriter;

impl ExportWriter for ToonWriter {
    /// Exports project analyses to a token-efficient TOON file.
    fn export(&self, projects: &[ProjectAnalysis], output_path: &Path) -> Result<()> {
        // Wrap projects with timestamp metadata
        let multi_analysis = ProjectsAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects.to_vec(),
        };

        // Serialize to TOON - minimal syntax, no redundant brackets or quotes
        let toon_content = serde_toon::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, toon_content)?;

        Ok(())
    }
}
