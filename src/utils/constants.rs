//! Application constants and configuration defaults
//!
//! This module defines global constants used throughout the application,
//! including config paths, ignore patterns, and size limits.

/// Config directory name
pub const CONFIG_DIR_NAME: &str = ".vibe-analyzer";

/// Config file name
pub const CONFIG_FILE_NAME: &str = "config.json";

/// Default ignore directories (if no .gitignore)
pub const DEFAULT_IGNORE_DIRS: &[&str] = &[
    "target",       // Rust
    "node_modules", // JavaScript/TypeScript
    "__pycache__",  // Python
    ".venv",        // Python virtual env
    "venv",         // Python virtual env
    ".git",         // Git
    ".idea",        // IntelliJ IDEA
];

/// Default ignore files (if no .gitignore)
pub const DEFAULT_IGNORE_FILES: &[&str] = &[
    ".DS_Store", // macOS
    "Thumbs.db", // Windows
    "*.hprof",   // Java heap dumps
    "*.log",     // Log files
];

/// Maximum file size for AST parsing (10 MB)
pub const MAX_AST_FILE_SIZE: u64 = 10 * 1024 * 1024;
