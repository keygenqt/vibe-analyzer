//! Indexer coordinator for OpenSearch operations.
//! Manages bulk indexing for meta, files, and analysis indices.

use crate::analyzer;
use crate::configs::app::AppConfig;
use crate::opensearch::client::OpenSearchClient;
use crate::print_info;
use anyhow::Result;

pub struct IndexReport {
    pub index_files: usize,
}

/// Index projects to OpenSearch.
pub async fn scan_index(
    config: &AppConfig,
    target: Option<String>,
    force: bool,
) -> Result<IndexReport> {
    let mut index_files = 0;

    // Connect to OpenSearch
    let client = OpenSearchClient::new(&config)?;
    if !client.healthcheck().await? {
        anyhow::bail!("OpenSearch is not available at {}", config.opensearch.host);
    }

    // Get enriched data
    let projects = analyzer::scan_analyze(config, target, force).await?;
    if projects.is_empty() {
        print_info!("No files to index");
        return Ok(IndexReport { index_files: 0 });
    }

    if force {
        // Full reindex: delete all indexes and start fresh
        print_info!("Clearing all indexes...");
        client.indexes_delete().await?;
    } else {
        // Incremental update: clean orphaned documents only
        print_info!("Cleaning up orphaned data...");
        client.indexes_cleanup().await?;
    }

    // Create or update indexes
    print_info!("Updating indexes...");
    client.indexes_create().await?;

    // Index projects
    for project in &projects {
        print_info!("Indexing: {}", project.path);
        client.index_update(project).await?;
        index_files += project.files.len();
    }

    Ok(IndexReport { index_files })
}
