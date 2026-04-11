//! OpenSearch connection configuration for indexing and querying analysis results.
//! Defines endpoint and authentication parameters.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for OpenSearch connection
/// TODO: Add authentication support (username, password, api_key, tls).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchConfig {
    /// OpenSearch endpoint URL
    pub host: String,
}

impl Default for OpenSearchConfig {
    /// Creates default OpenSearch configuration
    fn default() -> Self {
        Self {
            host: "http://localhost:9200".to_string(),
        }
    }
}
