//! OpenSearch connection configuration
//!
//! This module defines configuration settings for connecting to
//! an OpenSearch instance for indexing and searching project analysis data.

use serde::{Deserialize, Serialize};

/// Configuration for OpenSearch connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchConfig {
    /// OpenSearch endpoint URL
    pub host: String,

    /// Username for authentication
    pub username: String,

    /// Password for authentication
    pub password: String,
}

impl Default for OpenSearchConfig {
    /// Creates default OpenSearch configuration
    ///
    /// # Returns
    /// * `OpenSearchConfig` with default host, username, and password
    fn default() -> Self {
        Self {
            host: "http://localhost:9200".to_string(),
            username: "admin".to_string(),
            password: "password".to_string(),
        }
    }
}
