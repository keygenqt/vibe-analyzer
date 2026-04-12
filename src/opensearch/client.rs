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

#[derive(Clone)]
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

    pub fn base_url(&self) -> &str {
        &self.base_url
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
        let resp = self.client.head(&url).send().await?;
        Ok(resp.status() == StatusCode::OK)
    }

    pub async fn create_index(&self, index: &str, mapping: Value) -> Result<()> {
        let url = format!("{}/{}", self.base_url, index);
        self.client.put(&url).json(&mapping).send().await?;
        Ok(())
    }

    pub async fn delete_index(&self, index: &str) -> Result<()> {
        let url = format!("{}/{}", self.base_url, index);
        self.client.delete(&url).send().await?;
        Ok(())
    }

    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.client.post(url)
    }

    pub async fn bulk(&self, body: String) -> Result<Value> {
        let url = format!("{}/_bulk", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-ndjson")
            .body(body)
            .send()
            .await?;
        Ok(resp.json().await?)
    }

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

    pub async fn search(&self, index: &str, query: Value) -> Result<Value> {
        let url = format!("{}/{}/_search", self.base_url, index);
        Ok(self
            .client
            .post(&url)
            .json(&query)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Lists all index names from OpenSearch.
    pub async fn list_indices(&self) -> Result<Vec<String>> {
        let url = format!("{}/_cat/indices?format=json", self.base_url());
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to list indices")?;

        let indices: Vec<Value> = response.json().await?;

        Ok(indices
            .into_iter()
            .filter_map(|v| v["index"].as_str().map(String::from))
            .collect())
    }

    /// Deletes documents matching the given paths from an index.
    pub async fn delete_by_full_path(
        &self,
        index: &str,
        root: &str,
        paths: &[&str],
    ) -> Result<Value> {
        let should_clauses: Vec<Value> = paths
            .iter()
            .map(|path| {
                json!({
                    "bool": {
                        "must": [
                            { "term": { "Root": root } },
                            { "term": { "Path": path } }
                        ]
                    }
                })
            })
            .collect();

        let query = json!({
            "query": {
                "bool": {
                    "should": should_clauses,
                    "minimum_should_match": 1
                }
            }
        });

        self.delete_by_query(index, &query).await
    }

    /// Deletes documents matching a custom query.
    pub async fn delete_by_query(&self, index: &str, query: &Value) -> Result<Value> {
        let url = format!("{}/{}/_delete_by_query", self.base_url(), index);
        let response = self
            .client
            .post(&url)
            .json(query)
            .send()
            .await
            .context(format!("Failed to delete by query from index: {}", index))?;

        Ok(response.json().await?)
    }
}
