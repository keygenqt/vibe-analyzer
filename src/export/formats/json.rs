//! JSON format exporter for source analysis results.
//! Provides compact JSON output with minimal size (no extra whitespace).

use anyhow::Result;
use chrono::Local;
use serde_json;
use std::fs;
use std::path::Path;

use crate::analyzer::data::output::analyzer_project_output::AnalyzerProjectOutput;
use crate::analyzer::data::output::analyzer_projects_output::AnalyzerProjectsOutput;
use crate::scanner::data::output::scan_project_analysis_output::ScanProjectAnalysisOutput;
use crate::scanner::data::output::scan_projects_analysis_output::ScanProjectsAnalysisOutput;

use super::super::writer::ExportWriter;

/// JSON export writer implementing the ExportWriter trait.
pub struct JsonWriter;

impl ExportWriter for JsonWriter {
    /// Exports source analyses to a compact JSON file (minified).
    fn export_scanner(
        &self,
        sources: &[ScanProjectAnalysisOutput],
        output_path: &Path,
    ) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = ScanProjectsAnalysisOutput {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize without indentation for minimal size
        let json_content = serde_json::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json_content)?;

        Ok(())
    }

    /// Exports enriched source analyses to a compact JSON file.
    fn export_analyzer(&self, sources: &[AnalyzerProjectOutput], output_path: &Path) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = AnalyzerProjectsOutput {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize without indentation for minimal size
        let json_content = serde_json::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json_content)?;

        Ok(())
    }
}
