//! Core export functionality with pluggable format writers and registry.
//! Provides centralized export coordination and format selection.

use crate::export::formats::json::JsonWriter;
use crate::export::formats::json5::Json5Writer;
use crate::export::formats::toml::TomlWriter;
use crate::export::formats::toon::ToonWriter;
use crate::export::formats::xml::XmlWriter;
use crate::export::formats::yaml::YamlWriter;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use anyhow::Result;
use chrono::Local;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

/// Container for multiple source analyses (batch JSON export)
#[derive(Serialize)]
pub struct ExportModel {
    /// ISO 8601 timestamp of the batch export
    pub indexed_at: String,
    /// Collection of individual source analyses
    pub projects: Vec<ProjectModel>,
}

impl ExportModel {
    pub fn new_ast(projects: Vec<ProjectModel>) -> Self {
        ExportModel {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects
                .into_iter()
                .map(|p| ProjectModel {
                    files: FileModels::format_ast(p.files),
                    ..p
                })
                .collect(),
        }
    }

    pub fn new_analyze(projects: Vec<ProjectModel>) -> Self {
        ExportModel {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects
                .into_iter()
                .map(|p| ProjectModel {
                    files: FileModels::format_analyze(p.files),
                    ..p
                })
                .collect(),
        }
    }
}

/// Trait for implementing export format writers.
/// Each format must provide serialization logic for source analyses.
pub trait ExportWriter: Send + Sync {
    /// Exports source analyses to the specified file path.
    fn export(&self, sources: &ExportModel, output_path: &Path) -> Result<()>;
}

/// Registry of all available export writers, indexed by format type.
static WRITERS: Lazy<HashMap<ExportFormat, Box<dyn ExportWriter>>> = Lazy::new(|| {
    let mut map: HashMap<ExportFormat, Box<dyn ExportWriter>> = HashMap::new();
    map.insert(ExportFormat::Json, Box::new(JsonWriter));
    map.insert(ExportFormat::Json5, Box::new(Json5Writer));
    map.insert(ExportFormat::Toon, Box::new(ToonWriter));
    map.insert(ExportFormat::Xml, Box::new(XmlWriter));
    map.insert(ExportFormat::Yaml, Box::new(YamlWriter));
    map.insert(ExportFormat::Toml, Box::new(TomlWriter));
    map
});

/// Retrieves the export writer for the specified format.
fn get_writer(format: ExportFormat) -> Option<&'static dyn ExportWriter> {
    WRITERS.get(&format).map(|w| w.as_ref())
}

/// Generates the default export file path in the Downloads directory.
fn get_default_export(prefix: &String, format: ExportFormat) -> PathBuf {
    let timestamp = Local::now().format("%d-%m-%Y_%H-%M-%S");
    let filename = format!("{}-{}.{}", prefix, timestamp, format.extension());
    let downloads_dir = dirs::download_dir().expect("Failed to get downloads directory");
    downloads_dir.join(filename)
}

/// Export project model to file in specified format.
pub fn export(model: ExportModel, options: &ExportOptions) -> Result<PathBuf> {
    let writer =
        get_writer(options.format).ok_or_else(|| anyhow::anyhow!("Exporter is not registered."))?;

    let output_path = match options.output.clone() {
        Some(path) => vibe_fs::resolve(&path)
            .map_err(|e| anyhow::anyhow!("Error path for save output: {}", e))?,
        None => get_default_export(&options.prefix, options.format),
    };

    writer.export(&model, &output_path)?;
    Ok(output_path)
}
