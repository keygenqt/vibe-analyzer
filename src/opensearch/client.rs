//! OpenSearch HTTP client with retry logic and bulk operations.

use anyhow::Context;
use anyhow::Result;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::time::Duration;

use crate::opensearch::data::HasPath;

pub struct OpenSearchClient {
    client: Client,
    base_url: String,
}

impl OpenSearchClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub async fn healthcheck(&self) -> Result<bool> {
        match self
            .client
            .get(&self.base_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn index_exists(&self, index: &str) -> Result<bool> {
        let url = format!("{}/{}", self.base_url, index);
        let resp = self
            .client
            .head(&url)
            .send()
            .await
            .context("Failed to check index existence")?;
        Ok(resp.status() == StatusCode::OK)
    }

    pub async fn create_index(&self, index: &str, mapping: Value) -> Result<()> {
        let url = format!("{}/{}", self.base_url, index);
        self.client
            .put(&url)
            .json(&mapping)
            .send()
            .await
            .context("Failed to create index")?;
        Ok(())
    }

    pub async fn delete_index(&self, index: &str) -> Result<()> {
        let url = format!("{}/{}", self.base_url, index);
        self.client
            .delete(&url)
            .send()
            .await
            .context("Failed to delete index")?;
        Ok(())
    }

    pub async fn bulk(&self, body: String) -> Result<Value> {
        let url = format!("{}/_bulk", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-ndjson")
            .body(body)
            .send()
            .await
            .context("Failed to execute bulk request")?;

        resp.json().await.context("Failed to parse bulk response")
    }

    /// Bulk index documents using Path as _id.
    pub async fn bulk_index<T: Serialize + HasPath>(&self, index: &str, docs: &[&T]) -> Result<()> {
        if docs.is_empty() {
            return Ok(());
        }

        let mut body = String::new();
        for doc in docs {
            let action = json!({ "index": { "_index": index, "_id": doc.path() } });
            body.push_str(&serde_json::to_string(&action)?);
            body.push('\n');
            body.push_str(&serde_json::to_string(doc)?);
            body.push('\n');
        }

        let response = self.bulk(body).await?;

        if let Some(errors) = response.get("errors") {
            if errors.as_bool().unwrap_or(false) {
                anyhow::bail!(
                    "Bulk indexing failed: {}",
                    serde_json::to_string_pretty(&response)?
                );
            }
        }

        Ok(())
    }
}

impl Clone for OpenSearchClient {
    fn clone(&self) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to clone HTTP client"),
            base_url: self.base_url.clone(),
        }
    }
}
