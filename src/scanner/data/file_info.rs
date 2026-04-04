use std::path::PathBuf;

/// Represents a file discovered during directory scanning
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Absolute path to the file on disk
    pub path: PathBuf,
    /// Path relative to the project root being scanned
    pub relative_path: PathBuf,
    /// File size in bytes
    pub size: u64,
}
