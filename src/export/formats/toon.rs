//! TOON format exporter for source analysis results.
//! Provides token-efficient output optimized for LLM consumption.

use anyhow::Result;
use chrono::Local;
use serde_toon;
use std::fs;
use std::path::Path;

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::data::output::enriched_sources_analysis::EnrichedSourcesAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::sources_analysis::SourcesAnalysis;

use super::super::writer::ExportWriter;

/// TOON export writer implementing the ExportWriter trait.
pub struct ToonWriter;

impl ExportWriter for ToonWriter {
    /// Exports source analyses to a token-efficient TOON file.
    fn export_scanner(&self, sources: &[SourceAnalysis], output_path: &Path) -> Result<()> {
        // Wrap sources with timestamp metadata
        let multi_analysis = SourcesAnalysis {
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

        // Serialize to TOON - minimal syntax, no redundant brackets or quotes
        let toon_content = serde_toon::to_string(&multi_analysis)?;

        // Write to filesystem
        fs::write(output_path, toon_content)?;

        Ok(())
    }
}
