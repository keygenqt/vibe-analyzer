//! JSON5 format exporter for source analysis results.
//! Provides human-readable JSON5 output with comments and trailing commas support.

use anyhow::Result;
use chrono::Local;
use json5;
use std::fs;
use std::path::Path;

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::data::output::enriched_sources_analysis::EnrichedSourcesAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::sources_analysis::SourcesAnalysis;

use super::super::writer::ExportWriter;

/// JSON5 export writer implementing the ExportWriter trait.
pub struct Json5Writer;

impl ExportWriter for Json5Writer {
    /// Exports source analyses to a human-readable JSON5 file.
    fn export_scanner(&self, sources: &[SourceAnalysis], output_path: &Path) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = SourcesAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize with pretty formatting (json5 always produces readable output)
        let json5_content = json5::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json5_content)?;

        Ok(())
    }

    /// Exports enriched source analyses to a human-readable JSON5 file.
    fn export_analyzer(
        &self,
        sources: &[EnrichedSourceAnalysis],
        output_path: &Path,
    ) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = EnrichedSourcesAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize with pretty formatting (json5 always produces readable output)
        let json5_content = json5::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json5_content)?;

        Ok(())
    }
}
