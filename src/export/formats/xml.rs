//! XML format exporter with pretty printing support.

use anyhow::Result;
use serde_xml_rs::to_string;
use std::fs;
use std::path::Path;
use xml::reader::ParserConfig;
use xml::writer::EmitterConfig;

use crate::export::writer::ExportModel;
use crate::export::writer::ExportWriter;
use crate::models::file_models::FileModels;

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
            .perform_indent(true)
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
    fn export(&self, model: &ExportModel, output_path: &Path) -> Result<()> {
        let compact = to_string(model)?;
        let content = Self::pretty_print(&compact)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    /// Exports a single file model to a pretty-printed XML file.
    fn export_file(&self, source: &FileModels, output_path: &Path) -> Result<()> {
        let compact = to_string(source)?;
        let content = Self::pretty_print(&compact)?;
        fs::write(output_path, content)?;
        Ok(())
    }
}
