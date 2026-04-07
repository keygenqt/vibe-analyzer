//! Project statistics aggregation.
//! Tracks cumulative metrics across all scanned files including code counts and size information.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data::AstData;

/// Aggregated project statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ProjectStats {
    /// Total number of files
    pub total_files: usize,
    /// Number of text files
    pub text_files: usize,
    /// Number of binary files
    pub binary_files: usize,
    /// Total functions across all files
    pub total_functions: usize,
    /// Total classes across all files
    pub total_classes: usize,
    /// Total structs across all files
    pub total_structs: usize,
    /// Total enums across all files
    pub total_enums: usize,
    /// Total interfaces/traits/protocols across all files
    pub total_interfaces: usize,
    /// Total import statements across all files
    pub total_imports: usize,
    /// Total lines of code across all text files
    pub total_lines_of_code: usize,
    /// Total characters across all text files
    pub total_chars: usize,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Human readable size (e.g., "1.23 MB")
    pub total_size_human: String,
}

impl ProjectStats {
    /// Update stats with AST data from a single file
    pub fn update(&mut self, ast: Option<&AstData>) {
        self.total_files += 1;

        if let Some(ast_data) = ast {
            self.text_files += 1;

            // Functions
            self.total_functions += ast_data.functions.len();

            // Classes
            self.total_classes += ast_data.classes.len();

            // Structs
            self.total_structs += ast_data.structs.len();

            // Enums
            self.total_enums += ast_data.enums.len();

            // Interfaces/Traits/Protocols
            self.total_interfaces += ast_data.interfaces.len();

            // Imports
            self.total_imports += ast_data.imports.len();
        } else {
            self.binary_files += 1;
        }
    }

    /// Add lines of code to total
    pub fn add_lines_of_code(&mut self, lines: usize) {
        self.total_lines_of_code += lines;
    }

    /// Add characters to total
    pub fn add_chars(&mut self, chars: usize) {
        self.total_chars += chars;
    }

    /// Add file size and update human readable format
    pub fn add_size_bytes(&mut self, size: u64) {
        self.total_size_bytes += size;
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
