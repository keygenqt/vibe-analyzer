//! File models enum for serialization.
//! Represents the three file types in the pipeline: language, binary, text.

use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::nodes::nodes::Nodes;

use crate::models::file_base::FileBaseModel;
use crate::models::file_binary::FileBinaryModel;
use crate::models::file_language::FileLanguageModel;
use crate::models::file_text::FileTextModel;
use crate::models::trait_format_model::TraitFormatModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileModels {
    Language(FileLanguageModel),
    Binary(FileBinaryModel),
    Text(FileTextModel),
}

impl FileModels {
    /// Create binary file model from path.
    pub fn new_binary(path: impl AsRef<Path>) -> Self {
        FileModels::Binary(FileBinaryModel {
            base: FileBaseModel {
                path: path.as_ref().to_string_lossy().to_string(),
                ..Default::default()
            },
            extension: None,
        })
    }

    /// Create text file model from path.
    pub fn new_text(path: impl AsRef<Path>) -> Self {
        FileModels::Text(FileTextModel {
            base: FileBaseModel {
                path: path.as_ref().to_string_lossy().to_string(),
                content: std::fs::read_to_string(&path).ok(),
                ..Default::default()
            },
            preview: None,
            extension: None,
        })
    }

    /// Create language file model from path and parsed nodes.
    pub fn new_language(path: impl AsRef<Path>, nodes: Nodes) -> Self {
        FileModels::Language(FileLanguageModel::from(nodes).set_base(FileBaseModel {
            path: path.as_ref().to_string_lossy().to_string(),
            content: std::fs::read_to_string(&path).ok(),
            ..Default::default()
        }))
    }

    /// Apply format_ast to all models.
    pub fn format_ast(models: Vec<FileModels>) -> Vec<FileModels> {
        models
            .into_iter()
            .map(|m| match m {
                FileModels::Language(m) => FileModels::Language(m.format_ast()),
                FileModels::Binary(m) => FileModels::Binary(m.format_ast()),
                FileModels::Text(m) => FileModels::Text(m.format_ast()),
            })
            .collect()
    }

    /// Apply format_analyze to all models (AST + enrichment).
    pub fn format_analyze(models: Vec<FileModels>) -> Vec<FileModels> {
        models
            .into_iter()
            .map(|m| match m {
                FileModels::Language(m) => FileModels::Language(m.format_analyze()),
                FileModels::Binary(m) => FileModels::Binary(m.format_analyze()),
                FileModels::Text(m) => FileModels::Text(m.format_analyze()),
            })
            .collect()
    }
}
