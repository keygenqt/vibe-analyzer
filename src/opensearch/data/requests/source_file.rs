//! File content storage for OpenSearch indexing.
//! Contains raw file content for retrieval.

use anyhow::Result;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::opensearch::data::HasPath;

/// File content stored in vibe_files_{hash} index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    /// Absolute path to source root
    pub root: String,
    /// Relative path from source root (used as document _id)
    pub path: String,
    /// Raw file content
    pub content: String,
    /// Unique file identifier (BLAKE3 hash of content)
    pub hash: String,
}

impl HasPath for SourceFile {
    fn path(&self) -> &str {
        &self.path
    }
}

impl SourceFile {
    /// Creates SourceFile from root and relative path.
    pub fn new(root: &str, path: &str, hash: &str) -> Result<Self> {
        let full_path = Path::new(root).join(path);

        let content = fs::read(&full_path)
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
            .unwrap_or_else(|_| format!("[Binary file: {}]", path));

        Ok(Self {
            root: root.to_string(),
            path: path.to_string(),
            content,
            hash: hash.to_string(),
        })
    }

    /// Converts EnrichedSourceAnalysis to Vec<SourceFile>.
    pub fn from_enriched(source: &EnrichedSourceAnalysis) -> Result<Vec<Self>> {
        source
            .files
            .iter()
            .map(|f| Self::new(&source.root, &f.path, &f.hash))
            .collect()
    }
}
