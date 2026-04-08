//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file discovery, batching, and Ollama API calls.

use crate::{
    analyzer::{
        data::{
            output::enriched_project_analysis::EnrichedProjectAnalysis,
            requests::{
                ollama_file_request::OllamaFilesRequest,
                ollama_summary_request::OllamaSummaryRequest,
            },
        },
        ollama::OllamaClient,
    },
    scanner::data::output::project_analysis::ProjectAnalysis,
};
use anyhow::Result;
use chrono::Local;

/// Analyzes a project with LLM-powered enrichment.
pub async fn analyze_project(
    project: &ProjectAnalysis,
    ollama_client: &OllamaClient,
    on_progress: fn(usize, usize, usize),
    on_warning: fn(String),
) -> Result<EnrichedProjectAnalysis> {
    let total = project.files.len();
    // Step 1: Get enriched analysis project
    let summary = ollama_client
        .get_summary_enriched_analysis(&OllamaSummaryRequest::new(&project))
        .await?;

    // Step 2: Create project data
    let mut analysis = EnrichedProjectAnalysis {
        project: project.project.clone(),
        indexed_at: Local::now().to_rfc3339(),
        readme: project.readme.clone(),
        summary,
        license: project.license.clone(),
        stats: project.stats.clone(),
        files: Vec::with_capacity(total),
    };

    // Step 3: Get enriched analysis files
    let mut reqwest = OllamaFilesRequest::new(ollama_client.config.max_chunk_chars);
    for (i, file) in project.files.iter().enumerate() {
        let percent = (i + 1) as f64 / total as f64 * 100.0;
        on_progress(percent as usize, i + 1, total);

        if !reqwest.add(file) {
            // Send current batch
            if !reqwest.is_empty() {
                let file_enriched_analysis =
                    ollama_client.get_file_enriched_analysis(&reqwest).await?;
                analysis.files.extend(file_enriched_analysis);
            } else {
                on_warning(format!("File too large for LLM context: {}", file.path));
            }

            // Clear and start new batch with current file
            reqwest.clear();
            if !reqwest.add(file) {
                on_warning(format!("Failed to add file to new batch: {}", file.path));
            }
        }
    }

    // Don't forget the last batch!
    if !reqwest.is_empty() {
        let file_enriched_analysis = ollama_client.get_file_enriched_analysis(&reqwest).await?;
        analysis.files.extend(file_enriched_analysis);
    }

    Ok(analysis)
}
