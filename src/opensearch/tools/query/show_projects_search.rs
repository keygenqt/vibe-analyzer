//! Project listing query.

use anyhow::Result;
use serde_json::json;

use crate::{models::project::ProjectModel, opensearch::tools::tool_client::OpenSearchToolClient};

impl OpenSearchToolClient {
    /// Returns metadata for all indexed projects.
    pub async fn show_projects_search(&self, size: usize) -> Result<Vec<ProjectModel>> {
        let query = json!({
            "size": size,
            "query": { "match_all": {} }
        });

        let resp = self.call(query).await?;
        let hits = resp["hits"]["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        hits.iter()
            .map(|h| serde_json::from_value(h["_source"].clone()).map_err(anyhow::Error::from))
            .collect()
    }
}
