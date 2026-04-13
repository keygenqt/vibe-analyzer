//! File statistics extraction and aggregation.
//! Provides unified file metadata collection for AST parsing and analysis.

use anyhow::Result;

use crate::scanner::data::input::file_info::FileInfo;
use crate::utils;
use crate::utils::hash;
use crate::utils::path;

/// Collected statistics and metadata for a single file.
pub struct FileStats {
    /// File extension in lowercase (e.g., "rs", "py").
    pub extension: Option<String>,
    /// BLAKE3 hash of file content.
    pub hash: String,
    /// Whether the file is detected as text (not binary).
    pub is_text: bool,
    /// Number of lines in the file (0 for binary files).
    pub lines_of_code: usize,
    /// Number of characters in the file (0 for binary files).
    pub size_chars: usize,
    /// File size in bytes.
    pub size_bytes: u64,
}

impl FileStats {
    /// Extracts statistics and metadata from a file.
    pub fn new(file_info: &FileInfo) -> Result<FileStats> {
        let is_text = utils::check::is_text(&file_info.path);
        let hash = hash::hash_from_content(&file_info.path)?;
        let extension = path::get_extension(&file_info.path);
        let (lines_of_code, size_chars) = if is_text {
            match std::fs::read_to_string(&file_info.path) {
                Ok(content) => (content.lines().count(), content.chars().count()),
                Err(_) => (0, 0),
            }
        } else {
            (0, 0)
        };
        Ok(FileStats {
            extension,
            hash,
            is_text,
            lines_of_code,
            size_chars,
            size_bytes: file_info.size,
        })
    }
}
