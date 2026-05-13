//! Binary file model.
//! Represents a non-text file (images, archives, executables).

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::AnalyzeMode;
use crate::models::file_base::FileBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Binary file detected by content analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBinaryModel {
    /// Base file information (path, hash, content, bytes, lines).
    #[serde(flatten)]
    pub base: FileBaseModel,
    /// Optional file extension without dot.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
}

/// File model implementation for binary files.
impl TraitFileModel for FileBinaryModel {
    fn base_model(&self) -> &FileBaseModel {
        &self.base
    }
}

/// Format implementation for binary files.
impl TraitFormatModel for FileBinaryModel {
    fn format_ast(&self) -> Self {
        Self {
            base: self.base.format_ast(),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_analyze(&self) -> Self {
        Self {
            base: self.base.format_analyze(),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_index(&self) -> Self {
        Self {
            base: self.base.format_index(),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_tool(&self) -> Self {
        Self {
            base: self.base.format_tool(),
            extension: None,
        }
    }

    fn format_batch(&self, mode: AnalyzeMode) -> Self {
        Self {
            base: self.base.format_batch(mode),
            extension: self.extension.clone(),
        }
    }
}
