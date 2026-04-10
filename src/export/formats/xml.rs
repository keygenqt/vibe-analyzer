//! XML format exporter with pretty printing support.

use anyhow::Result;
use chrono::Local;
use serde_xml_rs::to_string;
use std::fs;
use std::path::Path;
use xml::reader::ParserConfig;
use xml::writer::EmitterConfig;

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::data::output::enriched_sources_analysis::EnrichedSourcesAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::sources_analysis::SourcesAnalysis;

use super::super::writer::ExportWriter;

/// XML export writer implementing the ExportWriter trait.
pub struct XmlWriter;

impl XmlWriter {
    /// Pretty-prints a raw XML string with proper indentation.
    fn pretty_print(xml: &str) -> Result<String> {
        let reader = ParserConfig::new()
            .trim_whitespace(true)
            .ignore_comments(false)
            .create_reader(xml.as_bytes());

        let mut output = Vec::new();
        let mut writer = EmitterConfig::new()
            .perform_indent(true) // Enables pretty indentation
            .normalize_empty_elements(false)
            .autopad_comments(false)
            .create_writer(&mut output);

        for event in reader {
            if let Some(event) = event?.as_writer_event() {
                writer.write(event)?;
            }
        }

        Ok(String::from_utf8(output)?)
    }
}

impl ExportWriter for XmlWriter {
    /// Exports source analyses to a pretty-printed XML file.
    fn export_scanner(&self, sources: &[SourceAnalysis], output_path: &Path) -> Result<()> {
        let multi_analysis = SourcesAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize to compact XML (no indentation)
        let compact_xml = to_string(&multi_analysis)?;

        // Pretty-print with indentation
        let pretty_xml = Self::pretty_print(&compact_xml)?;

        fs::write(output_path, pretty_xml)?;
        Ok(())
    }

    /// Exports enriched source analyses to a pretty-printed XML file.
    fn export_analyzer(
        &self,
        sources: &[EnrichedSourceAnalysis],
        output_path: &Path,
    ) -> Result<()> {
        let multi_analysis = EnrichedSourcesAnalysis {
            indexed_at: Local::now().to_rfc3339(),
            sources: sources.to_vec(),
        };

        // Serialize to compact XML (no indentation)
        let compact_xml = to_string(&multi_analysis)?;

        // Pretty-print with indentation
        let pretty_xml = Self::pretty_print(&compact_xml)?;

        fs::write(output_path, pretty_xml)?;
        Ok(())
    }
}
