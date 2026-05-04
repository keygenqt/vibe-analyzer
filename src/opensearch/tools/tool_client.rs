//! OpenSearch search client.
//! Generic search interface for all MCP tools.

use anyhow::Result;
use serde_json::{Value, json};

use crate::{
    models::{file_models::FileModels, project::ProjectModel},
    opensearch::client::{FILES_META_SUFFIX, OpenSearchClient, PROJECT_META_SUFFIX},
};

/// Client for executing search queries against project indices.
#[derive(Clone)]
pub struct OpenSearchToolClient {
    pub client: OpenSearchClient,
    pub names: Vec<String>,
}

impl OpenSearchToolClient {
    /// Creates new OpenSearchToolClient.
    pub fn new(client: &OpenSearchClient, names: Vec<String>) -> Self {
        Self {
            client: client.clone(),
            names,
        }
    }

    /// Executes a search query across all configured indices.
    pub async fn call(&self, query: Value) -> Result<Value> {
        let path = if self.names.is_empty() {
            "/_search".to_string()
        } else {
            format!("/{}/_search", self.names.join(","))
        };
        Ok(self
            .client
            .post(&path)
            .json(&query)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Fetch all project models from meta indices.
    pub async fn scroll_projects_meta(&self) -> Result<Vec<ProjectModel>> {
        if self.names.is_empty() {
            anyhow::bail!("No indices configured");
        }
        for name in &self.names {
            if !name.ends_with(PROJECT_META_SUFFIX) {
                anyhow::bail!("Index '{}' is not a project meta index", name);
            }
        }
        let mut all = Vec::new();
        for index in &self.names {
            all.extend(self.scroll_all::<ProjectModel>(index).await?);
        }
        Ok(all)
    }

    /// Fetch all file models from files_meta indices.
    pub async fn scroll_files_meta(&self) -> Result<Vec<FileModels>> {
        if self.names.is_empty() {
            anyhow::bail!("No indices configured");
        }
        for name in &self.names {
            if !name.contains(FILES_META_SUFFIX) {
                anyhow::bail!("Index '{}' is not a files analysis index", name);
            }
        }
        let mut all = Vec::new();
        for index in &self.names {
            all.extend(self.scroll_all::<FileModels>(index).await?);
        }
        Ok(all)
    }

    /// Fetch all documents from an index using scroll API.
    async fn scroll_all<T: serde::de::DeserializeOwned>(&self, index: &str) -> Result<Vec<T>> {
        let query = json!({
            "size": 1000,
            "query": { "match_all": {} }
        });
        let mut all_results = Vec::new();

        let mut resp: Value = self
            .client
            .post(&format!("/{}/_search?scroll=1m", index))
            .json(&query)
            .send()
            .await?
            .json()
            .await?;

        loop {
            let hits = match resp["hits"]["hits"].as_array() {
                Some(h) => h,
                None => break,
            };
            if hits.is_empty() {
                break;
            }
            for h in hits {
                if let Ok(doc) = serde_json::from_value::<T>(h["_source"].clone()) {
                    all_results.push(doc);
                }
            }
            let scroll_id = match resp["_scroll_id"].as_str() {
                Some(id) => id,
                None => break,
            };
            resp = self
                .client
                .post("/_search/scroll")
                .json(&json!({ "scroll": "1m", "scroll_id": scroll_id }))
                .send()
                .await?
                .json()
                .await?;
        }

        Ok(all_results)
    }
}
