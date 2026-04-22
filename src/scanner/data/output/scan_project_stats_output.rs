//! Source statistics aggregation.
//! Tracks cumulative metrics across all scanned files including code counts and size information.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::data::input::scan_file_stats_input::ScanFileStatsInput;

/// Aggregated source statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "ProjectStats")]
pub struct ScanProjectStatsOutput {
    /// Total number of files
    pub total_files: usize,
    /// Number of text files
    pub text_files: usize,
    /// Number of binary files
    pub binary_files: usize,
    /// Total lines of code across all text files
    pub total_lines_of_code: usize,
    /// Total characters across all text files
    pub total_chars: usize,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Human readable size (e.g., "1.23 MB")
    pub total_size_human: String,
}

impl ScanProjectStatsOutput {
    /// Updates aggregated statistics with data from a single file.
    pub fn add_stats(&mut self, stats: &ScanFileStatsInput) {
        self.total_files += 1;
        if stats.is_text {
            self.text_files += 1;
        } else {
            self.binary_files += 1;
        }
        self.total_lines_of_code += stats.lines_of_code;
        self.total_chars += stats.size_chars;
        self.total_size_bytes += stats.size_bytes;
        self.total_size_human = Self::format_size(self.total_size_bytes);
    }

    /// Format bytes as human readable string
    pub fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}
