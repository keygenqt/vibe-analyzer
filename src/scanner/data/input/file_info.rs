//! Internal file information structure.
//! Represents a file discovered during directory scanning with absolute and relative paths.

use std::path::PathBuf;

/// File discovered during directory scanning
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Absolute path on disk
    pub path: PathBuf,
    /// Path relative to source root
    pub relative_path: PathBuf,
    /// File size in bytes
    pub size: u64,
}
