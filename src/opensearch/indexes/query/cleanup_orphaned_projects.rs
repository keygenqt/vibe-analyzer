//! Cleanup orphaned projects from meta index.

use anyhow::Result;
use serde_json::{Value, json};

use crate::opensearch::indexes::indexes_client::OpenSearchIndexesClient;

impl OpenSearchIndexesClient {
    /// Delete projects not present in active roots.
    pub async fn cleanup_orphaned_projects(&self, active_roots: &[String]) -> Result<Value> {
        let query = json!({
            "query": {
                "bool": {
                    "must_not": {
                        "terms": { "path": active_roots }
                    }
                }
            }
        });

        self.delete_by_query(&query).await
    }
}
