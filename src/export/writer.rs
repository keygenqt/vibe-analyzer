//! Core export functionality with pluggable format writers and registry.
//! Provides centralized export coordination and format selection.

use crate::configs::export::ExportConfig;
use crate::export::formats::json::JsonWriter;
use crate::export::formats::json5::Json5Writer;
use crate::export::formats::toml::TomlWriter;
use crate::export::formats::toon::ToonWriter;
use crate::export::formats::xml::XmlWriter;
use crate::export::formats::yaml::YamlWriter;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::models::file_language_stats::FileLanguageStatsModel;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use crate::utils::app;
use crate::utils::path;
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
    /// ISO 8601 timestamp of the batch export.
    pub indexed_at: String,
    /// Collection of individual source analyses.
    pub projects: Vec<ProjectModel>,
}

impl ExportModel {
    /// Creates export with AST-only formatting, filtered by scan config.
    pub fn new_ast(config: &ExportConfig, projects: Vec<ProjectModel>) -> Self {
        let scan = config.clone();
        ExportModel {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects
                .into_iter()
                .map(|p| Self::filter_by_config(&scan, p))
                .map(|p| ProjectModel {
                    files: FileModels::format_ast(p.files),
                    ..p
                })
                .collect(),
        }
    }

    /// Creates export with AST + enrichment formatting, filtered by scan config.
    pub fn new_analyze(config: &ExportConfig, projects: Vec<ProjectModel>) -> Self {
        let scan = config.clone();
        ExportModel {
            indexed_at: Local::now().to_rfc3339(),
            projects: projects
                .into_iter()
                .map(|p| Self::filter_by_config(&scan, p))
                .map(|p| ProjectModel {
                    files: FileModels::format_analyze(p.files),
                    ..p
                })
                .collect(),
        }
    }

    /// Filter project files based on scan config.
    fn filter_by_config(config: &ExportConfig, project: ProjectModel) -> ProjectModel {
        ProjectModel {
            files: project
                .files
                .into_iter()
                .map(|f| match f {
                    FileModels::Language(mut lang) => {
                        if !config.include.imports {
                            lang.code.imports = vec![];
                        }
                        if !config.include.functions {
                            lang.code.functions = vec![];
                        }
                        if !config.include.variables {
                            lang.code.variables = vec![];
                        }
                        if !config.include.enums {
                            lang.code.enums = vec![];
                        }
                        if !config.include.interfaces {
                            lang.code.interfaces = vec![];
                        }
                        if !config.include.classes {
                            lang.code.classes = vec![];
                        }
                        if !config.include.structs {
                            lang.code.structs = vec![];
                        }
                        if !config.include.header_comments {
                            lang.code.header_comments = vec![];
                        }
                        if !config.include.headings {
                            lang.knowledge.headings = vec![];
                        }
                        if !config.include.links {
                            lang.knowledge.links = vec![];
                        }
                        if !config.include.code_blocks {
                            lang.knowledge.code_blocks = vec![];
                        }
                        if !config.include.stats {
                            lang.stats = FileLanguageStatsModel::default();
                        }
                        if !config.include.body {
                            for func in &mut lang.code.functions {
                                func.body = None;
                            }
                            for class in &mut lang.code.classes {
                                class.body = None;
                            }
                            for struct_ in &mut lang.code.structs {
                                struct_.body = None;
                            }
                            for enum_ in &mut lang.code.enums {
                                enum_.body = None;
                            }
                            for interface in &mut lang.code.interfaces {
                                interface.body = None;
                            }
                            for heading in &mut lang.knowledge.headings {
                                heading.body = None;
                            }
                        }
                        FileModels::Language(lang)
                    }
                    other => other,
                })
                .collect(),
            ..project
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

/// Generates the default export file path using config output directory.
fn get_default_export(format: &ExportFormat, output_dir: &PathBuf) -> Result<PathBuf> {
    let timestamp = Local::now().format("%d-%m-%Y_%H-%M-%S");
    let filename = format!("{}-{}.{}", app::app_name(), timestamp, format.extension());
    let dir = path::resolve_dir_path(Some(output_dir.clone()))?
        .ok_or_else(|| anyhow::anyhow!("Output directory not configured"))?;
    Ok(dir.join(filename))
}

/// Export project model to file in specified format.
pub fn export(
    model: ExportModel,
    options: &ExportOptions,
    config: &ExportConfig,
) -> Result<PathBuf> {
    let format = match options.format {
        Some(f) => f,
        None => ExportFormat::from_str(&config.format)
            .ok_or_else(|| anyhow::anyhow!(
                "Invalid export format in config: '{}'. Valid formats: json, json5, toon, xml, yaml, toml",
                config.format
            ))?,
    };
    let writer =
        get_writer(format).ok_or_else(|| anyhow::anyhow!("Exporter is not registered."))?;
    let output_path = match options.output.clone() {
        Some(path) => vibe_fs::resolve(&path)
            .map_err(|e| anyhow::anyhow!("Error path for save output: {}", e))?,
        None => get_default_export(&format, &config.output_dir)?,
    };
    writer.export(&model, &output_path)?;
    Ok(output_path)
}
