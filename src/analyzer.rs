//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file batching and parallel Ollama API calls.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::Result;
use colored::Colorize;
use futures::StreamExt;
use tokio::sync::{Mutex, mpsc};

use crate::configs::app::AppConfig;
use crate::{
    models::{file_language::FileLanguageModel, file_models::FileModels, project::ProjectModel},
    ollama::cluster::OllamaCluster,
};
use crate::{print_info, print_newline, print_progress, print_success, scanner};

/// Debug info for worker statistics.
struct WorkerStats {
    model: String,
    host: String,
    files_processed: Arc<AtomicUsize>,
}

/// Scan and enrich sources with LLM-powered analysis.
pub async fn scan_analyze(
    config: &AppConfig,
    target: Option<String>,
    force: bool,
) -> Result<Vec<ProjectModel>> {
    let cluster = OllamaCluster::new(&config.ollama);
    if cluster.is_empty() {
        anyhow::bail!("Ollama hosts not configured. Add to ~/.vibe-analyzer/config.json");
    }

    // Check all Ollama servers are ready
    cluster
        .is_ready(|current, total| {
            print_progress!("Checking Ollama server {}/{}", current, total);
            if total == current {
                print_newline!();
            }
        })
        .await
        .map_err(|e| {
            print_newline!();
            e
        })?;
    print_success!("All {} Ollama servers ready", cluster.len());

    // Scan sources
    let projects = scanner::scan_ast(config, target, force).await?;
    let total_files: usize = projects.iter().map(|p| p.files.len()).sum();
    if total_files == 0 {
        print_info!("No files to scan");
        return Ok(Vec::new());
    }
    print_success!("AST scanning completed: {} source(s)", projects.len());

    // Enrich each project
    let mut enriched = Vec::new();
    for project in projects {
        let name = project.path.split('/').last().unwrap_or(&project.path);
        print_info!("Start enrichment: {}", name.bold().blue());
        match analyze_project(
            &project,
            &cluster,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total);
                if total == current {
                    print_newline!();
                }
            },
            |message| print_info!("{}", message),
        )
        .await
        {
            Ok(analysis) => enriched.push(analysis),
            Err(e) => {
                print_newline!();
                return Err(e);
            }
        }
    }
    Ok(enriched)
}

/// Analyze a project with LLM-powered enrichment using parallel workers.
async fn analyze_project(
    project: &ProjectModel,
    cluster: &OllamaCluster,
    on_progress: fn(usize, usize, usize),
    on_done: fn(String),
) -> Result<ProjectModel> {
    // Collect only language files for enrichment
    let language_files: Vec<FileLanguageModel> = project
        .files
        .iter()
        .filter_map(|f| match f {
            FileModels::Language(m) => Some(m.clone()),
            _ => None,
        })
        .collect();

    let total = language_files.len();
    if total == 0 {
        return Ok(project.clone());
    }

    // Generate project summary from README
    let summary = if let Some(readme) = &project.readme {
        cluster.next().summary(&readme.content).await.ok()
    } else {
        None
    };

    let num_workers = cluster.len();
    let (tx, rx) = mpsc::channel::<Arc<FileLanguageModel>>(num_workers * 2);
    let shared_rx = Arc::new(Mutex::new(rx));
    let error_flag = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::with_capacity(num_workers);
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
            model: config.model.clone(),
            host: config.host.clone(),
            files_processed,
        });

        let handle = tokio::spawn(async move {
            let mut batch: Vec<FileLanguageModel> = Vec::new();
            let mut batch_chars = 0usize;
            let mut results: Vec<FileLanguageModel> = Vec::new();

            loop {
                if error_flag_clone.load(Ordering::Relaxed) {
                    return Ok(vec![]);
                }

                let file_opt = {
                    let mut rx_guard = rx.lock().await;
                    rx_guard.recv().await
                };

                let file_size = file_opt.as_ref().map_or(0, |f| {
                    serde_json::to_string(f).map(|s| s.len()).unwrap_or(0)
                });

                let is_last = file_opt.is_none();
                let is_full = !is_last
                    && (batch_chars + file_size > config.max_chunk_chars
                        || batch.len() >= config.max_chunk_files);
                let should_flush = !batch.is_empty() && (is_last || is_full);

                if should_flush {
                    match client.enrich_model(&batch).await {
                        Ok(enriched) => {
                            files_processed_clone.fetch_add(batch.len(), Ordering::Relaxed);
                            results.extend(enriched);
                            batch.clear();
                            batch_chars = 0;
                        }
                        Err(e) => {
                            error_flag_clone.store(true, Ordering::Relaxed);
                            return Err(e);
                        }
                    }
                }

                match file_opt {
                    Some(file) => {
                        batch_chars += file_size;
                        batch.push((*file).clone());
                    }
                    None => break,
                }
            }
            Ok::<_, anyhow::Error>(results)
        });
        abort_handles.push(handle.abort_handle());
        handles.push(handle);
    }

    // Send files to workers
    let tx = tx;
    let error_flag_send = error_flag.clone();
    tokio::spawn(async move {
        let mut sent = 0;
        for file in language_files {
            if error_flag_send.load(Ordering::Relaxed) {
                break;
            }
            if tx.send(Arc::new(file)).await.is_err() {
                break;
            }
            sent += 1;
            let percent = sent as f64 / total as f64 * 100.0;
            on_progress(percent as usize, sent, total);
        }
    });

    // Collect enriched files
    let mut enriched_files: Vec<FileLanguageModel> = Vec::with_capacity(total);
    let mut queue = handles
        .into_iter()
        .collect::<futures::stream::FuturesUnordered<_>>();
    while let Some(worker_result) = queue.next().await {
        match worker_result {
            Ok(Ok(files)) => enriched_files.extend(files),
            Ok(Err(e)) => {
                for h in &abort_handles {
                    h.abort();
                }
                return Err(e);
            }
            Err(e) => {
                for h in &abort_handles {
                    h.abort();
                }
                return Err(anyhow::anyhow!("Worker panicked: {}", e));
            }
        }
    }

    // Merge enriched files back into project
    let mut result = project.clone();
    result.summary = summary;

    for file in &mut result.files {
        if let FileModels::Language(lang) = file {
            if let Some(enriched) = enriched_files.iter().find(|e| e.path == lang.path) {
                lang.summary = enriched.summary.clone();
                lang.tags = enriched.tags.clone();
            }
        }
    }

    // Report worker stats
    if num_workers > 1 {
        let total_processed: usize = worker_stats
            .iter()
            .map(|s| s.files_processed.load(Ordering::Relaxed))
            .sum();
        for stat in &worker_stats {
            let processed = stat.files_processed.load(Ordering::Relaxed);
            let pct = if total_processed > 0 {
                (processed as f64 / total_processed as f64) * 100.0
            } else {
                0.0
            };
            on_done(format!(
                "Host {} ({}) processed: {} files ({:.1}%)",
                stat.host, stat.model, processed, pct
            ));
        }
    }

    Ok(result)
}
