//! File model trait.
//! Provides base model access and content preview.

use std::path::Path;

use crate::models::file_base::FileBaseModel;
use crate::models::trait_base_model::TraitBaseModel;

/// Trait for file models with base access and preview.
pub trait TraitFileModel {
    /// Returns reference to base file model.
    fn base_model(&self) -> &FileBaseModel;

    /// File extension without dot.
    fn extension(&self) -> Option<String> {
        Path::new(self.base_model().path())
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Content preview, breaking at word boundary. Default 100 characters.
    fn preview(&self, size: Option<usize>) -> Option<String> {
        let size = size.unwrap_or(100);
        self.base_model().content().and_then(|c| {
            let trimmed = c.trim_start_matches(|ch| ch == ' ' || ch == '\n');
            let chars: Vec<char> = trimmed.chars().take(size).collect();
            if chars.is_empty() {
                return None;
            }
            let preview: String = if let Some(pos) = chars.iter().rposition(|&ch| ch == ' ') {
                chars[..pos].iter().collect()
            } else {
                chars.iter().collect()
            };
            let preview = preview.trim_end();
            if preview.is_empty() {
                None
            } else {
                Some(preview.to_string())
            }
        })
    }
}
