//! Project model.
//! Represents a scanned project with metadata and files.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use tokio::runtime::Handle;
use vibe_ast::Language;

use crate::models::file_models::FileModels;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::utils::spdx::LicenseDetector;

/// Project-level metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseModel {
    /// Path to license file.
    pub path: String,
    /// SPDX identifier.
    pub spdx_id: String,
    /// Detection confidence (0.0 - 1.0).
    pub confidence: f64,
}

/// Aggregated project statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsModel {
    /// Total number of files.
    pub total_files: usize,
    /// Number of text files.
    pub text_files: usize,
    /// Number of binary files.
    pub binary_files: usize,
    /// Total characters.
    pub total_chars: usize,
    /// Total size in bytes.
    pub total_size_bytes: u64,
    /// Total number of lines.
    pub total_size_lines: usize,
}

/// Scanned project with all metadata and files.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let mut total_size_bytes = 0u64;
        let mut total_size_lines = 0;
        let mut total_chars = 0;
        for file in files {
            match file {
                FileModels::Language(lang) => {
                    text_files += 1;
                    let base = lang.base_model();
                    let size = base.size_bytes.or_else(|| base.size_bytes()).unwrap_or(0);
                    total_size_bytes += size;
                    total_size_lines += base.size_lines.or_else(|| base.size_lines()).unwrap_or(0);
                    total_chars += size as usize;
                }
                FileModels::Text(text) => {
                    text_files += 1;
                    let base = text.base_model();
                    let size = base.size_bytes.or_else(|| base.size_bytes()).unwrap_or(0);
                    total_size_bytes += size;
                    total_size_lines += base.size_lines.or_else(|| base.size_lines()).unwrap_or(0);
                    total_chars += size as usize;
                }
                FileModels::Binary(bin) => {
                    binary_files += 1;
                    let base = bin.base_model();
                    total_size_bytes += base.size_bytes.or_else(|| base.size_bytes()).unwrap_or(0);
                }
            }
        }
        StatsModel {
            total_files,
            text_files,
            binary_files,
            total_chars,
            total_size_bytes,
            total_size_lines,
        }
    }

    /// Detect README file from scanned files.
    fn detect_readme(files: &[FileModels]) -> Option<ReadmeModel> {
        files
            .iter()
            .filter_map(|f| match f {
                FileModels::Language(lang) if lang.node_language == Some(Language::Markdown) => {
                    let name = lang.base_model().name().to_lowercase();
                    if name.contains("readme") {
                        lang.preview(Some(500))
                            .map(|preview| (lang.base_model().path().len(), preview, lang))
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
                content: lang
                    .base_model()
                    .content()
                    .map(|c| c.to_string())
                    .unwrap_or_default(),
                frontmatter: lang.knowledge.frontmatter.clone(),
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
                    license_names.contains(&m.base_model().name().to_lowercase().as_str())
                }
                FileModels::Text(m) => {
                    license_names.contains(&m.base_model().name().to_lowercase().as_str())
                }
                _ => false,
            })
            // Build license models with SPDX detection
            .filter_map(|model| {
                let (path, content) = match model {
                    FileModels::Language(m) => {
                        (m.base_model().path(), m.base_model().content.clone())
                    }
                    FileModels::Text(m) => (m.base_model().path(), m.base_model().content.clone()),
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
