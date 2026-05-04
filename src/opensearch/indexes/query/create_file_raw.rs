//! Create file_raw index for raw file content storage.

use anyhow::Result;
use serde_json::json;

use crate::opensearch::indexes::index_client::OpenSearchIndexClient;

impl OpenSearchIndexClient {
    /// Create vibe_files_{hash} index with mapping for file content.
    pub async fn create_file_raw(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "dynamic": "strict",
                "properties": {
                    "path": { "type": "keyword" },
                    "content": { "type": "text", "index": false, "store": true }
                }
            }
        });

        self.index_create(mapping).await
    }
}
