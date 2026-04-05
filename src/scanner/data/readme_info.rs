//! README file information
//!
//! This module defines [`ReadmeInfo`] containing content and metadata
//! of a project's README file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadmeInfo {
    /// Path to README file
    pub path: String,
    /// Full content of README file
    pub content: String,
    /// File size in bytes
    pub size_bytes: u64,
}
