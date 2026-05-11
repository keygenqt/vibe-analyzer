//! Raw file model.
//! Contains full file content for storage and retrieval.

use serde::Deserialize;
use serde::Serialize;

use crate::models::file_base::FileBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Raw file with full content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTextModel {
    /// Base file information (path, hash, content, bytes, lines).
    #[serde(flatten)]
    pub base: FileBaseModel,
    /// First N bytes of file content for preview.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub preview: Option<String>,
    /// Optional file extension without dot.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
}

/// File model implementation for text files.
impl TraitFileModel for FileTextModel {
    fn base_model(&self) -> &FileBaseModel {
        &self.base
    }
}

/// Format implementation for text files.
impl TraitFormatModel for FileTextModel {
    fn format_ast(&self) -> Self {
        Self {
            base: self.base.format_ast(),
            preview: self.preview.clone().or_else(|| self.preview(None)),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_analyze(&self) -> Self {
        Self {
            base: self.base.format_analyze(),
            preview: self.preview.clone().or_else(|| self.preview(None)),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_index(&self) -> Self {
        Self {
            base: self.base.format_index(),
            preview: self.preview.clone().or_else(|| self.preview(None)),
            extension: self.extension.clone().or_else(|| self.extension()),
        }
    }

    fn format_tool(&self) -> Self {
        Self {
            base: self.base.format_tool(),
            preview: self.preview.clone().or_else(|| self.preview(None)),
            extension: None,
        }
    }
}
