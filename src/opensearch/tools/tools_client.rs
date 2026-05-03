//! @todo
//! @todo

use anyhow::Result;
use serde_json::Value;

use crate::opensearch::client::OpenSearchClient;

/// @todo
#[derive(Clone)]
pub struct OpenSearchToolsClient {
    pub client: OpenSearchClient,
}

impl OpenSearchToolsClient {
    /// Creates new OpenSearchToolsClient.
    pub fn new(client: OpenSearchClient) -> Self {
        Self { client }
    }

    /// @todo
    pub async fn call(&self, index: &str, query: Value) -> Result<Value> {
        let url = format!("{}/{}/_search", self.client.base_url, index);
        Ok(self
            .client
            .post(&url)
            .json(&query)
            .send()
            .await?
            .json()
            .await?)
    }
}
