//! Internal file information structure
//!
//! This module defines [`FileInfo`] used during directory scanning
//! before full analysis is performed.

use std::path::PathBuf;

/// File discovered during directory scanning
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Absolute path on disk
    pub path: PathBuf,
    /// Path relative to project root
    pub relative_path: PathBuf,
    /// File size in bytes
    pub size: u64,
}
