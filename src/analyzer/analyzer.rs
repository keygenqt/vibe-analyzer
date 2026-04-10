//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file discovery, batching, and Ollama API calls.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::analyzer::data::requests::ollama_files_request::OllamaFilesRequest;
use crate::analyzer::data::requests::ollama_summary_request::OllamaSummaryRequest;
use crate::analyzer::ollama_cluster::OllamaCluster;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::{
    analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis,
    scanner::data::output::file_analysis::FileAnalysis,
};
use anyhow::Result;
use chrono::Local;
use tokio::sync::{Mutex, mpsc};

/// Debug info for worker statistics.
struct WorkerStats {
    model: String,
    files_processed: Arc<AtomicUsize>,
}

/// Analyzes a source with LLM-powered enrichment using file-level distribution.
/// Fast workers naturally process more files, maximizing cluster utilization.
pub async fn analyze_source(
    source: &SourceAnalysis,
    cluster: &OllamaCluster,
    on_progress: fn(usize, usize, usize),
    on_warning: fn(String),
    on_done: fn(String),
) -> Result<EnrichedSourceAnalysis> {
    let total = source.files.len();

    // Step 1: Get enriched analysis source
    let summary = cluster
        .next()
        .get_summary_enriched_analysis(&OllamaSummaryRequest::new(&source))
        .await?;

    // Step 2: Create source data
    let mut analysis = EnrichedSourceAnalysis {
        source: source.source.clone(),
        indexed_at: Local::now().to_rfc3339(),
        readme: source.readme.clone(),
        summary,
        license: source.license.clone(),
        stats: source.stats.clone(),
        files: Vec::with_capacity(total),
    };

    // Step 3: Setup parallel workers with shared receiver
    let num_workers = cluster.len();
    let (file_tx, file_rx) = mpsc::channel::<Arc<FileAnalysis>>(num_workers * 2);
    let shared_rx = Arc::new(Mutex::new(file_rx));
    let mut handles = Vec::with_capacity(num_workers);

    // @todo
    let mut worker_stats: Vec<WorkerStats> = Vec::with_capacity(num_workers);

    // Spawn workers that compete for files from the shared receiver
    for worker_id in 0..num_workers {
        let client = cluster.clients[worker_id].clone();
        let rx = shared_rx.clone();
        let max_chunk = client.config.max_chunk_chars;
        let on_warning = on_warning.clone();
        let files_processed = Arc::new(AtomicUsize::new(0));
        let files_processed_clone = files_processed.clone();

        worker_stats.push(WorkerStats {
            model: client.config.model.clone(),
            files_processed,
        });

        let handle = tokio::spawn(async move {
            let mut batch = OllamaFilesRequest::new();
            let mut results = Vec::new();

            loop {
                // Compete for the next file
                let file = {
                    let mut rx_guard = rx.lock().await;
                    rx_guard.recv().await
                };

                match file {
                    Some(file) => {
                        if !batch.add(&file, max_chunk) {
                            // Batch is full - send it
                            if !batch.is_empty() {
                                match client.get_file_enriched_analysis(&batch).await {
                                    Ok(enriched) => {
                                        files_processed_clone
                                            .fetch_add(batch.files.len(), Ordering::Relaxed);
                                        results.extend(enriched);
                                    }
                                    Err(e) => return Err(e),
                                }
                            }

                            // Start new batch with current file
                            batch.clear();
                            if !batch.add(&file, max_chunk) {
                                on_warning(format!(
                                    "File too large for LLM context: {}",
                                    file.path
                                ));
                            }
                        }
                    }
                    None => break, // Channel closed, no more files
                }
            }

            // Send last batch
            if !batch.is_empty() {
                match client.get_file_enriched_analysis(&batch).await {
                    Ok(enriched) => {
                        files_processed_clone.fetch_add(batch.files.len(), Ordering::Relaxed);
                        results.extend(enriched);
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok::<_, anyhow::Error>(results)
        });
        handles.push(handle);
    }

    // Step 4: Send all files to the shared channel
    let mut sent = 0;
    for file in source.files.iter() {
        file_tx.send(Arc::new(file.clone())).await?;
        sent += 1;
        let percent = sent as f64 / total as f64 * 100.0;
        on_progress(percent as usize, sent, total);
    }

    // Close the sender - workers will finish after processing all files
    drop(file_tx);

    // Step 5: Collect results from all workers
    for handle in handles {
        match handle.await {
            Ok(Ok(enriched)) => analysis.files.extend(enriched),
            Ok(Err(e)) => return Err(e),
            Err(e) => return Err(anyhow::anyhow!("Worker panicked: {}", e)),
        }
    }

    // @todo
    if num_workers > 1 {
        let total_processed: usize = worker_stats
            .iter()
            .map(|s| s.files_processed.load(Ordering::Relaxed))
            .sum();

        for stat in &worker_stats {
            let processed = stat.files_processed.load(Ordering::Relaxed);
            let percentage = if total_processed > 0 {
                (processed as f64 / total_processed as f64) * 100.0
            } else {
                0.0
            };
            on_done(format!(
                "Model '{}' processed: {} files ({:.1}%)",
                stat.model, processed, percentage
            ));
        }
    }

    Ok(analysis)
}
