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
}
