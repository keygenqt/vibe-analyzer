//! OpenSearch indexes client.
//! Provides index management and document operations.

use anyhow::Result;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

use crate::opensearch::client::OpenSearchClient;

/// Client for managing OpenSearch indices and documents.
#[derive(Clone)]
pub struct OpenSearchIndexClient {
    pub client: OpenSearchClient,
    pub name: String,
}

impl OpenSearchIndexClient {
    /// Creates new OpenSearchIndexClient.
    pub fn new(client: &OpenSearchClient, name: String) -> Self {
        Self {
            client: client.clone(),
            name,
        }
    }

    /// Check if index exists.
    pub async fn index_exists(&self) -> Result<bool> {
        let resp = self.client.head(&format!("/{}", self.name)).send().await?;
        Ok(resp.status() == StatusCode::OK)
    }

    /// Create index with mapping.
    pub async fn index_create(&self, mapping: Value) -> Result<()> {
        self.client
            .put(&format!("/{}", self.name))
            .json(&mapping)
            .send()
            .await?;
        Ok(())
    }

    /// Delete index.
    pub async fn index_delete(&self) -> Result<()> {
        self.client
            .delete(&format!("/{}", self.name))
            .send()
            .await?;
        Ok(())
    }

    /// Index multiple documents in bulk.
    pub async fn bulk<T: Serialize>(&self, docs: &[(String, T)]) -> Result<()> {
        if docs.is_empty() {
            return Ok(());
        }

        let mut body = String::new();
        for (path, doc) in docs {
            let action = json!({ "index": { "_index": self.name, "_id": path } });
            body.push_str(&serde_json::to_string(&action)?);
            body.push('\n');
            body.push_str(&serde_json::to_string(doc)?);
            body.push('\n');
        }

        let resp = self
            .client
            .post("/_bulk")
            .header("Content-Type", "application/x-ndjson")
            .body(body)
            .send()
            .await?;

        let response: Value = resp.json().await?;

        if response
            .get("errors")
            .and_then(|e| e.as_bool())
            .unwrap_or(false)
        {
            anyhow::bail!(
                "Bulk indexing failed: {}",
                serde_json::to_string_pretty(&response)?
            );
        }

        Ok(())
    }

    /// Delete documents matching a query.
    pub async fn delete_by_query(&self, query: &Value) -> Result<Value> {
        let response = self
            .client
            .post(&format!("/{}/_delete_by_query", self.name))
            .json(query)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete from index {}: {}", self.name, e))?;
        Ok(response.json().await?)
    }
}
