//! Cleanup orphaned documents from index.

use std::path::PathBuf;

use anyhow::Result;
use serde_json::{Value, json};

use crate::opensearch::indexes::index_client::OpenSearchIndexClient;

impl OpenSearchIndexClient {
    /// Delete orphaned documents not in active paths.
    pub async fn cleanup_orphaned_files(&self, paths: &[PathBuf]) -> Result<Value> {
        let paths: Vec<String> = paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        let query = json!({
            "query": {
                "bool": {
                    "must_not": {
                        "terms": { "path": paths }
                    }
                }
            }
        });
        self.delete_by_query(&query).await
    }
}
