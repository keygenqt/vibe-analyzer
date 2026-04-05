use serde::{Deserialize, Serialize};

use crate::scanner::ast::parser::AstData;

/// Aggregated statistics for a project analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectStats {
    /// Total number of files processed
    pub total_files: usize,
    /// Number of text files (source code, configs, etc.)
    pub text_files: usize,
    /// Number of binary files (images, compiled binaries, etc.)
    pub binary_files: usize,
    /// Total number of functions extracted from all files
    pub total_functions: usize,
    /// Total number of classes extracted from all files
    pub total_classes: usize,
    /// Total number of structs extracted from all files
    pub total_structs: usize,
    /// Total number of enums extracted from all files
    pub total_enums: usize,
    /// Total number of traits/interfaces extracted from all files
    pub total_traits: usize,
    /// Total number of import statements extracted from all files
    pub total_imports: usize,
    /// Total number of lines of code across all text files
    pub total_lines_of_code: usize,
    /// Total number of characters across all text files
    pub total_chars: usize,
    /// Total size of all files in bytes
    pub total_size_bytes: u64,
    /// Human readable total size (e.g., "1.23 MB", "456 KB")
    pub total_size_human: String,
}

impl ProjectStats {
    /// Updates statistics based on AST data from a single file
    pub fn update(&mut self, ast: Option<&AstData>) {
        self.total_files += 1;

        if let Some(ast_data) = ast {
            self.text_files += 1;
            self.total_functions += ast_data.functions.len();
            self.total_classes += ast_data.classes.len();
            self.total_structs += ast_data.structs.len();
            self.total_enums += ast_data.enums.len();
            self.total_traits += ast_data.traits.len();
            self.total_imports += ast_data.imports.len();
        } else {
            self.binary_files += 1;
        }
    }

    /// Adds lines of code to the total
    pub fn add_lines_of_code(&mut self, lines: usize) {
        self.total_lines_of_code += lines;
    }

    /// Adds characters to the total
    pub fn add_chars(&mut self, chars: usize) {
        self.total_chars += chars;
    }

    /// Adds file size to the total and updates human readable format
    pub fn add_size_bytes(&mut self, size: u64) {
        self.total_size_bytes += size;
        self.total_size_human = Self::format_size(self.total_size_bytes);
    }

    /// Formats bytes into human readable string (KB, MB, GB)
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
