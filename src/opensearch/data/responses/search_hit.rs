//! Base search hit model.
//! Contains common metadata for all OpenSearch search results.

use serde::Deserialize;
use serde::Serialize;

/// Base metadata included in every search hit.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHit {
    /// Source root path.
    pub root: String,
    /// Index name where the match was found.
    pub index: String,
    /// Relative file path.
    pub path: String,
}
