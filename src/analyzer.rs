//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file batching and parallel Ollama API calls.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use anyhow::Result;
use clap::ValueEnum;
use colored::Colorize;
use futures::StreamExt;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

use crate::configs::app::AppConfig;
use crate::models::file_language::FileLanguageModel;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;
use crate::ollama::cluster::OllamaCluster;
use crate::print_info;
use crate::print_newline;
use crate::print_progress;
use crate::print_success;
use crate::scanner;

/// Enrichment mode determines what analysis to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum AnalyzeMode {
    /// AST structure only, no LLM enrichment.
    Ast,
    /// File summary and search tags.
    Meta,
    /// Technical debt: TODO, magic numbers, commented-out code.
    Debt,
    /// Refactoring and test suggestions.
    Advice,
    /// Bugs and spelling errors.
    Errors,
}

impl AnalyzeMode {
    /// Human-readable mode name.
    pub fn name(&self) -> &'static str {
        match self {
            AnalyzeMode::Ast => "AST",
            AnalyzeMode::Meta => "Meta",
            AnalyzeMode::Debt => "Debt",
            AnalyzeMode::Advice => "Advice",
            AnalyzeMode::Errors => "Errors",
        }
    }
}

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
    path: Option<PathBuf>,
    modes: &Vec<AnalyzeMode>,
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
        .await?;
    print_success!("All {} Ollama servers ready", cluster.len());

    // Scan sources
    let projects = scanner::scan_ast(config, target, path, force).await?;
    let total_files: usize = projects.iter().map(|p| p.files.len()).sum();
    if total_files == 0 {
        print_info!("No files to analyze");
        return Ok(Vec::new());
    }
    print_success!("AST scanning completed: {} source(s)", projects.len());

    // Enrich each project
    let mut enriched = Vec::new();
    for mut project in projects {
        let name = project.path.split('/').last().unwrap_or(&project.path);
        print_info!("Start enrichment source: {}", name.bold().blue());
        for mode in modes {
            // Use smaller batch for non-Meta modes to avoid context overflow
            let max_chunk_files = match mode {
                AnalyzeMode::Meta => 5,
                _ => 1,
            };
            match analyze_project(
                &project,
                &cluster,
                mode.clone(),
                config.analyze.max_chunk_chars,
                max_chunk_files,
                |mode, percent, current, total| {
                    print_progress!(
                        "Processing enrich {}... {:.1}% ({}/{})",
                        mode.name(),
                        percent,
                        current,
                        total
                    );
                    if total == current {
                        print_newline!();
                    }
                },
                |message| print_info!("{}", message),
            )
            .await
            {
                Ok(analysis) => {
                    // Keep first non-empty summary from any enrichment mode
                    project.summary = analysis.summary.or(project.summary);
                    // Merge enrichment results back into the same project
                    for (file, enriched_file) in project.files.iter_mut().zip(analysis.files.iter())
                    {
                        match (file, enriched_file) {
                            (FileModels::Language(lang), FileModels::Language(enriched_lang)) => {
                                lang.analyze.merge(&enriched_lang.analyze);
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    print_newline!();
                    return Err(e);
                }
            }
        }
        enriched.push(project);
    }
    Ok(enriched)
}

/// Analyze a project with LLM-powered enrichment using parallel workers.
async fn analyze_project(
    project: &ProjectModel,
    cluster: &OllamaCluster,
    mode: AnalyzeMode,
    max_chunk_chars: usize,
    max_chunk_files: usize,
    on_progress: fn(AnalyzeMode, usize, usize, usize),
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
            let mut batch_chars = 0usize;
            let mut batch: Vec<FileLanguageModel> = Vec::new();
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
                    && (batch_chars + file_size > max_chunk_chars
                        || batch.len() >= max_chunk_files);
                let should_flush = !batch.is_empty() && (is_last || is_full);

                if should_flush {
                    match client.enrich_model(mode, &batch).await {
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
            if tx.send(Arc::new(file.format_batch(mode))).await.is_err() {
                break;
            }
            sent += 1;
            let percent = sent as f64 / total as f64 * 100.0;
            on_progress(mode, percent as usize, sent, total);
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
            if let Some(enriched) = enriched_files
                .iter()
                .find(|e| e.base_model().path() == lang.base_model().path())
            {
                lang.analyze.merge(&enriched.analyze);
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
