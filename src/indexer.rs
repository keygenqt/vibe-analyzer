//! Indexer coordinator for OpenSearch operations.
//! Manages bulk indexing for meta, files, and analysis indices.

use std::path::PathBuf;

use anyhow::Result;
use serde_json::json;

use crate::{
    analyzer,
    configs::app::AppConfig,
    models::{file_model::FileModel, file_models::FileModels, project::ProjectModel},
    opensearch::client::OpenSearchClient,
    print_info,
    utils::app,
};

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

    let client = OpenSearchClient::new(config)?;
    if !client.healthcheck().await? {
        anyhow::bail!("OpenSearch is not available at {}", config.opensearch.host);
    }

    let projects = analyzer::scan_analyze(config, target, force).await?;
    if projects.is_empty() {
        print_info!("No files to index");
        return Ok(IndexReport { index_files: 0 });
    }

    if force {
        print_info!("Clearing all indexes...");
        indexes_delete(&client).await?;
    } else {
        print_info!("Cleaning up orphaned data...");
        indexes_cleanup(&config, &client).await?;
    }

    print_info!("Updating indexes...");
    indexes_create(&config, &client).await?;

    for project in &projects {
        print_info!("Indexing: {}", project.path);
        index_update(&client, project).await?;
        index_files += project.files.len();
    }

    Ok(IndexReport { index_files })
}

/// Clean up orphaned documents across all indexes.
async fn indexes_cleanup(config: &AppConfig, client: &OpenSearchClient) -> Result<()> {
    let sources = config.get_sources();
    let active_roots: Vec<String> = sources
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();

    let meta = client.client_index(&client.projects_meta_index());
    if meta.index_exists().await? {
        meta.cleanup_orphaned_projects(&active_roots).await?;
    }

    for source in &sources {
        let active_files = vibe_fs::scan(source, true)?;

        let analysis = client.client_index(&client.files_meta_index(source));
        if analysis.index_exists().await? {
            analysis.cleanup_orphaned_files(&active_files).await?;
        }

        let files = client.client_index(&client.files_index(source));
        if files.index_exists().await? {
            files.cleanup_orphaned_files(&active_files).await?;
        }
    }

    let all_indices = client.indexes_list().await?;
    let expected: Vec<String> = sources
        .iter()
        .flat_map(|s| [client.files_index(s), client.files_meta_index(s)])
        .chain(std::iter::once(client.projects_meta_index()))
        .collect();

    for index in all_indices {
        if index.starts_with(&format!("{}-", app::app_name())) && !expected.contains(&index) {
            client.client_index(&index).index_delete().await?;
        }
    }

    Ok(())
}

/// Create all required indexes if they don't exist.
async fn indexes_create(config: &AppConfig, client: &OpenSearchClient) -> Result<()> {
    let sources = config.get_sources();

    let meta = client.client_index(&client.projects_meta_index());
    if !meta.index_exists().await? {
        meta.create_project_meta().await?;
    }

    for source in &sources {
        let files_meta = client.client_index(&client.files_meta_index(source));
        if !files_meta.index_exists().await? {
            files_meta.create_file_meta().await?;
        }
        let files = client.client_index(&client.files_index(source));
        if !files.index_exists().await? {
            files.create_file_raw().await?;
        }
    }
    Ok(())
}

/// Index a project: meta, file analysis, and file content.
async fn index_update(client: &OpenSearchClient, project: &ProjectModel) -> Result<()> {
    let root = PathBuf::from(&project.path);

    let meta = client.client_index(&client.projects_meta_index());
    let meta_json = serde_json::to_value(project.format_index())?;
    meta.bulk(&[(project.path.clone(), meta_json)]).await?;

    let files_meta = client.client_index(&client.files_meta_index(&root));
    let meta_docs: Vec<(String, serde_json::Value)> = project
        .files
        .iter()
        .map(|f| match f {
            FileModels::Language(m) => m.format_index(),
            FileModels::Text(m) => m.format_index(),
            FileModels::Binary(m) => m.format_index(),
        })
        .map(|f| {
            let path = match &f {
                FileModels::Language(m) => m.path().to_string(),
                FileModels::Text(m) => m.path().to_string(),
                FileModels::Binary(m) => m.path().to_string(),
            };
            let json = serde_json::to_value(&f)?;
            Ok((path, json))
        })
        .collect::<Result<Vec<_>>>()?;
    files_meta.bulk(&meta_docs).await?;

    let files_raw = client.client_index(&client.files_index(&root));
    let raw_docs: Vec<(String, serde_json::Value)> = project
        .files
        .iter()
        .filter_map(|f| {
            let path = match f {
                FileModels::Language(m) => m.path(),
                FileModels::Text(m) => m.path(),
                FileModels::Binary(_) => return None,
            };
            let content = match f {
                FileModels::Language(m) => m.content.clone(),
                FileModels::Text(m) => m.content.clone(),
                FileModels::Binary(_) => None,
            };
            content.map(|c| (path.to_string(), json!({"path": path, "content": c})))
        })
        .collect();
    files_raw.bulk(&raw_docs).await?;

    Ok(())
}

/// Delete all indexes for this application.
async fn indexes_delete(client: &OpenSearchClient) -> Result<()> {
    let all_indices = client.indexes_list().await?;
    for index in all_indices {
        if index.starts_with(&format!("{}-", app::app_name())) {
            client.client_index(&index).index_delete().await?;
        }
    }
    Ok(())
}
