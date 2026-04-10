//! JSON format exporter for source analysis results.
//! Provides compact JSON output with minimal size (no extra whitespace).

use anyhow::Result;
use chrono::Local;
use serde_json;
use std::fs;
use std::path::Path;

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::data::output::enriched_sources_analysis::EnrichedSourcesAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::sources_analysis::SourcesAnalysis;

use super::super::writer::ExportWriter;

/// JSON export writer implementing the ExportWriter trait.
pub struct JsonWriter;

impl ExportWriter for JsonWriter {
    /// Exports source analyses to a compact JSON file (minified).
    fn export_scanner(&self, sources: &[SourceAnalysis], output_path: &Path) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = SourcesAnalysis {
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

        // Serialize without indentation for minimal size
        let json_content = serde_json::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, json_content)?;

        Ok(())
    }
}
