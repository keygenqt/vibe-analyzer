//! File hash response model for incremental indexing.
//! Contains file path and content hash from OpenSearch.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// File hashes for a specific source.
#[derive(Debug)]
pub struct SearchFileHashSource {
    /// File hashes for this source (full path to file -> hash).
    pub hashes: HashMap<String, String>,
}

/// Response containing file path and its content hash.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchFileHash {
    /// Relative file path.
    pub path: String,
    /// BLAKE3 hash of file content.
    pub hash: String,
}
