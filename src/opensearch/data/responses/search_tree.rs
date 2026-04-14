//! Search result structure for directory tree queries.
//! Contains formatted tree string and counts of directories and files.

use serde::Deserialize;
use serde::Serialize;

/// Directory tree result returned from tree search queries.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchTree {
    /// Tree structure as formatted string.
    pub tree: String,
    /// Total number of directories within the specified depth.
    pub directories: usize,
    /// Total number of files within the specified depth.
    pub files: usize,
}
