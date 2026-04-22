//! README file detection for source documentation.
//! Scans for README files and prioritizes Markdown over plain text.

use crate::scanner::data::input::scan_file_info_input::ScanFileInfoInput;
use crate::scanner::data::output::scan_readme_output::ScanReadmeOutput;

/// README file detector
pub struct ReadmeDetector;

impl ReadmeDetector {
    /// Finds the best README file (priority: .md > .txt > no extension)
    pub fn find_readme(files: &[ScanFileInfoInput]) -> Option<ScanReadmeOutput> {
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
                .map(|content| ScanReadmeOutput {
                    path: file.relative_path.to_string_lossy().to_string(),
                    content: content.chars().collect(),
                    size_bytes: file.size,
                })
        })
    }
}

/// Convenience function for README detection
pub fn detect_readme(files: &[ScanFileInfoInput]) -> Option<ScanReadmeOutput> {
    ReadmeDetector::find_readme(files)
}
