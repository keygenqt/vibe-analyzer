use crate::scanner::data::{file_info::FileInfo, readme_info::ReadmeInfo};

/// Detects and extracts README files from a list of project files
pub struct ReadmeDetector;

impl ReadmeDetector {
    /// Finds the best README file in the file list
    ///
    /// # Priority order (highest to lowest)
    /// 1. `README.md` - Markdown files have highest priority
    /// 2. `README.txt` - Plain text files
    /// 3. `README` - Files without extension
    ///
    /// # Recognized patterns
    /// - `readme`, `README`, `Readme`
    ///
    /// # Returns
    /// Returns `Some(ReadmeInfo)` for the first matching file,
    /// `None` if no README file is found.
    pub fn find_readme(files: &[FileInfo]) -> Option<ReadmeInfo> {
        let readme_patterns = ["readme", "README", "Readme"];

        // Collect all files matching README patterns
        let mut candidates: Vec<_> = files
            .iter()
            .filter(|f| {
                let stem = f.path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
                readme_patterns.contains(&stem)
            })
            .collect();

        // Sort by priority: .md > .txt > no extension
        candidates.sort_by_key(|f| {
            let ext = f.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            match ext {
                "md" => 1,  // Highest priority
                "txt" => 2, // Medium priority
                _ => 3,     // Lowest priority
            }
        });

        // Take the best candidate and read its content
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
pub fn detect_readme(files: &[FileInfo]) -> Option<ReadmeInfo> {
    ReadmeDetector::find_readme(files)
}
