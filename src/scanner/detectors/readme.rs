//! README file detection
//!
//! This module provides functionality to find and extract README files
//! from a list of project files, with priority for .md over .txt.

use crate::scanner::data::{file_info::FileInfo, readme_info::ReadmeInfo};

/// README file detector
pub struct ReadmeDetector;

impl ReadmeDetector {
    /// Finds the best README file (priority: .md > .txt > no extension)
    ///
    /// # Arguments
    /// * `files` - List of files to search for README
    ///
    /// # Returns
    /// * `Option<ReadmeInfo>` - README information if found
    pub fn find_readme(files: &[FileInfo]) -> Option<ReadmeInfo> {
        let readme_patterns = ["readme", "README", "Readme"];

        let mut candidates: Vec<_> = files
            .iter()
            .filter(|f| {
                let stem = f.path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
                readme_patterns.contains(&stem)
            })
            .collect();

        candidates.sort_by_key(|f| {
            let ext = f.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            match ext {
                "md" => 1,
                "txt" => 2,
                _ => 3,
            }
        });

        candidates.first().and_then(|file| {
            std::fs::read_to_string(&file.path)
                .ok()
                .map(|content| ReadmeInfo {
                    path: file.relative_path.to_string_lossy().to_string(),
                    content: content.chars().collect(),
                    size_bytes: file.size,
                })
        })
    }
}

/// Convenience function for README detection
///
/// # Arguments
/// * `files` - List of files to search for README
///
/// # Returns
/// * `Option<ReadmeInfo>` - README information if found
pub fn detect_readme(files: &[FileInfo]) -> Option<ReadmeInfo> {
    ReadmeDetector::find_readme(files)
}
