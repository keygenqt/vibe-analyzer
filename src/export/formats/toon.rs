//! TOON format exporter for source analysis results.
//! Provides token-efficient output optimized for LLM consumption.

use anyhow::Result;
use chrono::Local;
use serde_toon;
use std::fs;
use std::path::Path;

use crate::analyzer::data::output::analyzer_project_output::AnalyzerProjectOutput;
use crate::analyzer::data::output::analyzer_projects_output::AnalyzerProjectsOutput;
use crate::scanner::data::output::scan_project_analysis_output::ScanProjectAnalysisOutput;
use crate::scanner::data::output::scan_projects_analysis_output::ScanProjectsAnalysisOutput;

use super::super::writer::ExportWriter;

/// TOON export writer implementing the ExportWriter trait.
pub struct ToonWriter;

impl ExportWriter for ToonWriter {
    /// Exports source analyses to a token-efficient TOON file.
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

        // Serialize to TOON - minimal syntax, no redundant brackets or quotes
        let toon_content = serde_toon::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, toon_content)?;

        Ok(())
    }

    /// Exports enriched source analyses to a token-efficient TOON file.
    fn export_analyzer(&self, sources: &[AnalyzerProjectOutput], output_path: &Path) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = AnalyzerProjectsOutput {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize to TOON - minimal syntax, no redundant brackets or quotes
        let toon_content = serde_toon::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, toon_content)?;

        Ok(())
    }
}
