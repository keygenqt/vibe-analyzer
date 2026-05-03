//! Project model.
//! Represents a scanned project with metadata and files.

use std::{collections::HashMap, path::Path};

use serde::Serialize;
use tokio::runtime::Handle;
use vibe_ast::Language;

use crate::{
    models::{file_model::FileModel, file_models::FileModels},
    utils::spdx::LicenseDetector,
};

/// Project-level metadata.
#[derive(Debug, Clone, Serialize)]
pub struct ReadmeModel {
    /// README content preview.
    pub preview: String,
    /// YAML frontmatter key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub frontmatter: Option<HashMap<String, String>>,
    /// Number of characters in preview.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_chars: Option<usize>,
    /// Number of bytes in preview.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_bytes: Option<usize>,
    /// Detected programming or markup language.
    #[serde(skip)]
    pub content: String,
}

/// License information.
#[derive(Debug, Clone, Serialize)]
pub struct LicenseModel {
    /// Path to license file.
    pub path: String,
    /// SPDX identifier.
    pub spdx_id: String,
    /// Detection confidence (0.0 - 1.0).
    pub confidence: f64,
}

/// Aggregated project statistics.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StatsModel {
    /// Total number of files.
    pub total_files: usize,
    /// Number of text files.
    pub text_files: usize,
    /// Number of binary files.
    pub binary_files: usize,
    /// Total lines of code.
    pub total_lines_of_code: usize,
    /// Total characters.
    pub total_chars: usize,
    /// Total size in bytes.
    pub total_size_bytes: u64,
}

/// Scanned project with all metadata and files.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectModel {
    /// Project root path.
    pub path: String,
    /// Detected README.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub readme: Option<ReadmeModel>,
    /// LLM-generated project summary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub summary: Option<String>,
    /// Detected license files.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub licenses: Vec<LicenseModel>,
    /// Aggregated statistics.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stats: Option<StatsModel>,
    /// All scanned files.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<FileModels>,
}

impl ProjectModel {
    /// Create project model from root path and scanned files.
    pub fn new(path: impl AsRef<Path>, files: Vec<FileModels>) -> Self {
        let readme = Self::detect_readme(&files);
        let licenses = Self::detect_licenses(&files);
        let stats = Self::compute_stats(&files);
        Self {
            path: path.as_ref().to_string_lossy().to_string(),
            readme,
            summary: None,
            licenses,
            stats: Some(stats),
            files,
        }
    }

    /// Format project for meta index (without files).
    pub fn format_index(&self) -> Self {
        ProjectModel {
            path: self.path.clone(),
            readme: self.readme.clone(),
            summary: self.summary.clone(),
            licenses: self.licenses.clone(),
            stats: self.stats.clone(),
            files: Vec::new(),
        }
    }

    /// Compute aggregated statistics from file list.
    fn compute_stats(files: &[FileModels]) -> StatsModel {
        let total_files = files.len();
        let mut text_files = 0;
        let mut binary_files = 0;
        let mut total_lines_of_code = 0;
        let mut total_chars = 0;
        let mut total_size_bytes = 0u64;
        for file in files {
            match file {
                FileModels::Language(lang) => {
                    text_files += 1;
                    total_lines_of_code += lang
                        .lines_of_code
                        .or_else(|| lang.lines_of_code())
                        .unwrap_or(0);
                    let size = lang.size_bytes.or_else(|| lang.size_bytes()).unwrap_or(0);
                    total_size_bytes += size;
                    total_chars += size as usize;
                }
                FileModels::Text(text) => {
                    text_files += 1;
                    let size = text.size_bytes.or_else(|| text.size_bytes()).unwrap_or(0);
                    total_size_bytes += size;
                    total_chars += size as usize;
                }
                FileModels::Binary(bin) => {
                    binary_files += 1;
                    total_size_bytes += bin.size_bytes.or_else(|| bin.size_bytes()).unwrap_or(0);
                }
            }
        }
        StatsModel {
            total_files,
            text_files,
            binary_files,
            total_lines_of_code,
            total_chars,
            total_size_bytes,
        }
    }

    /// Detect README file from scanned files.
    fn detect_readme(files: &[FileModels]) -> Option<ReadmeModel> {
        files
            .iter()
            .filter_map(|f| match f {
                FileModels::Language(lang) if lang.node_language == Some(Language::Markdown) => {
                    let name = lang.name().to_lowercase();
                    if name.contains("readme") {
                        lang.preview(Some(500))
                            .map(|preview| (lang.path().len(), preview, lang))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .min_by_key(|(len, _, _)| *len)
            .map(|(_, preview, lang)| ReadmeModel {
                size_chars: Some(preview.chars().count()),
                size_bytes: Some(preview.len()),
                preview,
                content: lang.content.clone().unwrap_or_default(),
                frontmatter: lang.frontmatter.clone(),
            })
    }

    /// Detect license files from scanned files.
    fn detect_licenses(files: &[FileModels]) -> Vec<LicenseModel> {
        let license_names = ["license", "copying"];
        files
            .iter()
            // Filter files matching license names
            .filter(|f| match f {
                FileModels::Language(m) if m.node_language == Some(Language::Markdown) => {
                    license_names.contains(&m.name().to_lowercase().as_str())
                }
                FileModels::Text(m) => license_names.contains(&m.name().to_lowercase().as_str()),
                _ => false,
            })
            // Build license models with SPDX detection
            .filter_map(|model| {
                let (path, content) = match model {
                    FileModels::Language(m) => (m.path(), m.content.clone()),
                    FileModels::Text(m) => (m.path(), m.content.clone()),
                    _ => return None,
                };
                content
                    .as_ref()
                    .and_then(|c| {
                        tokio::task::block_in_place(|| {
                            Handle::current().block_on(LicenseDetector::detect(c))
                        })
                    })
                    .map(|info| LicenseModel {
                        path: path.to_string(),
                        spdx_id: info.spdx_id,
                        confidence: info.confidence,
                    })
            })
            .collect()
    }
}
