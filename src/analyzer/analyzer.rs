//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file discovery, batching, and Ollama API calls.

use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::data::requests::ollama_files_request::OllamaFilesRequest;
use crate::analyzer::data::requests::ollama_summary_request::OllamaSummaryRequest;
use crate::analyzer::ollama_cluster::OllamaCluster;
use crate::scanner::data::output::file_analysis::FileAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use anyhow::Result;
use chrono::Local;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

/// Debug info for worker statistics.
struct WorkerStats {
    model: String,
    host: String,
    files_processed: Arc<AtomicUsize>,
}

/// Analyzes a source with LLM-powered enrichment using file-level distribution.
/// Fast workers naturally process more files, maximizing cluster utilization.
pub async fn analyze_source(
    source: &SourceAnalysis,
    cluster: &OllamaCluster,
    on_progress: fn(usize, usize, usize),
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
        root: source.root.clone(),
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

    // Spawn workers that compete for files from the shared receiver
    let error_flag = Arc::new(AtomicBool::new(false));
    let mut abort_handles = Vec::new();
    let mut worker_stats: Vec<WorkerStats> = Vec::with_capacity(num_workers);
    for worker_id in 0..num_workers {
        let client = cluster.clients[worker_id].clone();
        let rx = shared_rx.clone();
        let config = client.config.clone();
        let files_processed = Arc::new(AtomicUsize::new(0));
        let files_processed_clone = files_processed.clone();
        let error_flag_clone = error_flag.clone();

        worker_stats.push(WorkerStats {
            model: client.config.model.clone(),
            host: client.config.host.clone(),
            files_processed,
        });

        let handle = tokio::spawn(async move {
            let mut batch = OllamaFilesRequest::new();
            let mut results = Vec::new();

            loop {
                if error_flag_clone.load(Ordering::Relaxed) {
                    return Ok(vec![]);
                }

                let file_opt = {
                    let mut rx_guard = rx.lock().await;
                    rx_guard.recv().await
                };

                let is_last = file_opt.is_none();
                let is_full = !is_last && !batch.can_fit(file_opt.as_ref().unwrap(), &config)
                    || batch.files.len() >= config.max_chunk_files;
                let should_flush = !batch.is_empty() && (is_last || is_full);

                if should_flush {
                    match client.get_file_enriched_analysis(&batch).await {
                        Ok(enriched) => {
                            files_processed_clone.fetch_add(batch.files.len(), Ordering::Relaxed);
                            results.extend(enriched);
                            batch.clear();
                        }
                        Err(e) => {
                            error_flag_clone.store(true, Ordering::Relaxed);
                            return Err(e);
                        }
                    }
                }

                match file_opt {
                    Some(file) => {
                        if !batch.add(&file, &config) {
                            anyhow::bail!("File '{}' too large", file.path);
                        }
                    }
                    None => break,
                }
            }
            Ok::<_, anyhow::Error>(results)
        });
        abort_handles.push(handle.abort_handle());
        handles.push(handle);
    }

    // Step 4: Send all files to the shared channel in a background task
    let file_tx = file_tx;
    let source_files = source.files.clone();
    let error_flag_send = error_flag.clone();
    let total = source_files.len();
    tokio::spawn(async move {
        let mut sent = 0;
        for file in source_files {
            if error_flag_send.load(Ordering::Relaxed) {
                break;
            }
            if file_tx.send(Arc::new(file)).await.is_err() {
                break;
            }
            sent += 1;
            let percent = sent as f64 / total as f64 * 100.0;
            on_progress(percent as usize, sent, total);
        }
    });

    // Step 5: Collect results from all workers
    let mut queue = handles
        .into_iter()
        .collect::<futures::stream::FuturesUnordered<_>>();
    while let Some(worker_result) = queue.next().await {
        match worker_result {
            Ok(Ok(enriched)) => analysis.files.extend(enriched),
            Ok(Err(e)) => {
                for abort_h in abort_handles {
                    abort_h.abort();
                }
                return Err(e);
            }
            Err(e) => {
                for abort_h in &abort_handles {
                    abort_h.abort();
                }
                return Err(anyhow::anyhow!(
                    "Worker task panicked or was cancelled: {}",
                    e
                ));
            }
        }
    }

    // Report per-worker statistics when multiple Ollama hosts are used
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
                "Host {} ({}) processed: {}",
                stat.host,
                stat.model,
                format!("{} files ({:.1}%)", processed, percentage)
            ));
        }
    }

    Ok(analysis)
}
