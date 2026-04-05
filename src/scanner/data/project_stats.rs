//! Project statistics aggregation
//!
//! This module defines [`ProjectStats`] for aggregating metrics
//! across all files in a project.

use serde::{Deserialize, Serialize};

use crate::scanner::ast::parser::AstData;

/// Aggregated project statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    /// Total traits/interfaces across all files
    pub total_traits: usize,
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
    ///
    /// # Arguments
    /// * `ast` - Optional AST data from a file
    ///
    /// # Returns
    /// * `()` - Unit type (updates struct in place)
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

    /// Add lines of code to total
    ///
    /// # Arguments
    /// * `lines` - Number of lines to add
    ///
    /// # Returns
    /// * `()` - Unit type (updates struct in place)
    pub fn add_lines_of_code(&mut self, lines: usize) {
        self.total_lines_of_code += lines;
    }

    /// Add characters to total
    ///
    /// # Arguments
    /// * `chars` - Number of characters to add
    ///
    /// # Returns
    /// * `()` - Unit type (updates struct in place)
    pub fn add_chars(&mut self, chars: usize) {
        self.total_chars += chars;
    }

    /// Add file size and update human readable format
    ///
    /// # Arguments
    /// * `size` - File size in bytes to add
    ///
    /// # Returns
    /// * `()` - Unit type (updates struct in place)
    pub fn add_size_bytes(&mut self, size: u64) {
        self.total_size_bytes += size;
        self.total_size_human = Self::format_size(self.total_size_bytes);
    }

    /// Format bytes as human readable string
    ///
    /// # Arguments
    /// * `bytes` - Size in bytes to format
    ///
    /// # Returns
    /// * `String` - Human readable size string (e.g., "1.23 MB")
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
