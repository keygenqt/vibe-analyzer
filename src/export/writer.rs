//! Core export functionality with pluggable format writers and registry.
//! Provides centralized export coordination and format selection.

use crate::export::formats::json::JsonWriter;
use crate::export::formats::json5::Json5Writer;
use crate::export::formats::toon::ToonWriter;
use crate::export::formats::xml::XmlWriter;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::utils::path;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

/// Trait for implementing export format writers.
/// Each format must provide serialization logic for project analyses.
pub trait ExportWriter: Send + Sync {
    /// Exports project analyses to the specified file path.
    fn export(&self, projects: &[ProjectAnalysis], output_path: &Path) -> Result<()>;
}

/// Registry of all available export writers, indexed by format type.
static WRITERS: Lazy<HashMap<ExportFormat, Box<dyn ExportWriter>>> = Lazy::new(|| {
    let mut map: HashMap<ExportFormat, Box<dyn ExportWriter>> = HashMap::new();

    map.insert(ExportFormat::Json, Box::new(JsonWriter));
    map.insert(ExportFormat::Json5, Box::new(Json5Writer));
    map.insert(ExportFormat::Toon, Box::new(ToonWriter));
    map.insert(ExportFormat::Xml, Box::new(XmlWriter));

    map
});

/// Retrieves the export writer for the specified format.
fn get_writer(format: ExportFormat) -> Option<&'static dyn ExportWriter> {
    WRITERS.get(&format).map(|w| w.as_ref())
}

/// Exports project analyses using the specified options.
/// Returns the absolute path to the created export file.
pub fn export_projects(projects: Vec<ProjectAnalysis>, options: &ExportOptions) -> Result<PathBuf> {
    // Look up the writer for the requested format
    let writer =
        get_writer(options.format).ok_or_else(|| anyhow::anyhow!("Exporter is not registered."))?;

    // Resolve output path: use custom path or generate default
    let output_path = match options.output.clone() {
        Some(path) => path::convert_relative(&path.to_string_lossy())
            .ok_or_else(|| anyhow::anyhow!("Error path for save output."))?,
        None => path::get_default_export(options.format),
    };

    // Perform the actual export
    writer.export(&projects, &output_path)?;

    Ok(output_path)
}
