//! File models enum for serialization.
//! Represents the three file types in the pipeline: language, binary, text.

use std::path::Path;

use serde::Serialize;
use vibe_ast::nodes::nodes::Nodes;

use crate::models::{
    file_binary::FileBinaryModel, file_language::FileLanguageModel, file_model::FileModel,
    file_text::FileTextModel,
};

#[derive(Debug, Clone, Serialize)]
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
            path: path.as_ref().to_string_lossy().to_string(),
            extension: None,
            size_bytes: None,
            hash: None,
        })
    }

    /// Create text file model from path.
    pub fn new_text(path: impl AsRef<Path>) -> Self {
        FileModels::Text(FileTextModel {
            path: path.as_ref().to_string_lossy().to_string(),
            content: std::fs::read_to_string(&path).ok(),
            preview: None,
            extension: None,
            size_bytes: None,
            hash: None,
        })
    }

    /// Create language file model from path and parsed nodes.
    pub fn new_language(path: impl AsRef<Path>, nodes: Nodes) -> Self {
        let content = std::fs::read_to_string(&path).ok();
        FileModels::Language(
            FileLanguageModel::from(nodes)
                .with_path(path)
                .with_content(content),
        )
    }

    /// Apply format_ast to all models.
    pub fn format_ast(models: Vec<FileModels>) -> Vec<FileModels> {
        models
            .into_iter()
            .map(|m| match m {
                FileModels::Language(m) => m.format_ast(),
                FileModels::Binary(m) => m.format_ast(),
                FileModels::Text(m) => m.format_ast(),
            })
            .collect()
    }

    /// Apply format_analyze to all models (AST + enrichment).
    pub fn format_analyze(models: Vec<FileModels>) -> Vec<FileModels> {
        models
            .into_iter()
            .map(|m| match m {
                FileModels::Language(m) => m.format_analyze(),
                FileModels::Binary(m) => m.format_analyze(),
                FileModels::Text(m) => m.format_analyze(),
            })
            .collect()
    }
}
