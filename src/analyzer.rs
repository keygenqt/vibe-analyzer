//! Core analyzer that orchestrates scanning and LLM enrichment.
//! Coordinates file batching and parallel cluster API calls.

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use clap::ValueEnum;
use colored::Colorize;
use serde_json::Value;
use vibe_ast::Language;
use vibe_cluster::Cluster;
use vibe_cluster::ConnectionParams;
use vibe_cluster::Prompt;
use vibe_cluster::Provider;

use crate::configs::app::AppConfig;
use crate::enrich::DefaultEnrich;
use crate::enrich::EnrichPrompt;
use crate::models::file_language::FileLanguageModel;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;
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
    /// Host URL (e.g. "http://localhost:11434").
    pub host: String,
    /// Model name (e.g. "qwen2.5-coder:3b-instruct").
    pub model: String,
    /// Provider type (e.g. "Ollama").
    pub provider: String,
    /// Number of files processed by this worker.
    pub files_processed: usize,
}

/// Scan and enrich sources with LLM-powered analysis.
pub async fn scan_analyze(
    config: &AppConfig,
    target: Option<String>,
    path: Option<PathBuf>,
    modes: &[AnalyzeMode],
    force: bool,
) -> Result<Vec<ProjectModel>> {
    // Build cluster from config
    let connections: Vec<ConnectionParams> = config
        .cluster
        .iter()
        .map(|c| {
            let provider = match c.provider.as_str() {
                "ollama" => Provider::Ollama,
                "deepseek" => Provider::DeepSeek,
                "qwen" => Provider::Qwen,
                other => anyhow::bail!("Unknown provider: {}", other),
            };
            Ok(ConnectionParams {
                provider,
                host: c.host.clone(),
                model: c.model.clone(),
                temperature: Some(c.temperature),
                seed: Some(c.seed),
                num_ctx: Some(c.num_ctx),
                num_predict: Some(c.num_predict),
                timeout_secs: Some(c.timeout_secs),
                api_key: c.api_key.clone(),
                parallel: c.parallel,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let cluster = Cluster::new(connections.clone())?;

    // Check all servers are ready
    print_info!("Checking cluster nodes...");
    cluster.is_available().await?;
    print_success!("All {} cluster nodes ready", connections.len());

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
            print_info!("Preparing {} enrichment...", mode.name());
            match analyze_project(
                &project,
                &cluster,
                *mode,
                config.analyze.max_chunk_chars,
                |m, percent, current, total| {
                    print_progress!(
                        "Processing enrich {}... {:.1}% ({}/{})",
                        m.name(),
                        percent,
                        current,
                        total
                    );
                    if total == current {
                        print_newline!();
                    }
                },
                |stats| {
                    for s in stats {
                        print_info!(
                            "[{}] Host {} ({}) processed: {} files",
                            s.provider,
                            s.host,
                            s.model,
                            s.files_processed
                        );
                    }
                },
            )
            .await
            {
                Ok(analysis) => {
                    project = analysis;
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

/// Analyze a project with LLM-powered enrichment using vibe-cluster.
async fn analyze_project(
    project: &ProjectModel,
    cluster: &Cluster,
    mode: AnalyzeMode,
    max_chunk_chars: usize,
    on_progress: fn(AnalyzeMode, usize, usize, usize),
    on_done: fn(Vec<WorkerStats>),
) -> Result<ProjectModel> {
    if mode == AnalyzeMode::Ast {
        anyhow::bail!("AST mode does not use LLM enrichment");
    }

    let language_files: Vec<FileLanguageModel> = project
        .files
        .iter()
        .filter_map(|f| match f {
            FileModels::Language(m) => Some(m.clone()),
            _ => None,
        })
        .collect();

    let total_files = language_files.len();
    if total_files == 0 {
        return Ok(project.clone());
    }

    // Summary
    let summary = if let Some(readme) = &project.readme {
        cluster
            .call(Prompt::new(format!(
                "Summarize this README in 2-3 sentences.\n\nLANGUAGE: English only.\n\n{}",
                readme.content
            )))
            .await
            .ok()
            .and_then(|r| r.text)
    } else {
        None
    };

    // Build prompts
    let mut vibe_prompts: Vec<Prompt> = Vec::new();

    if mode == AnalyzeMode::Meta {
        let mut chunk: Vec<FileLanguageModel> = Vec::new();
        let mut formatted_chunk: Vec<FileLanguageModel> = Vec::new();
        let mut chunk_chars = 0;

        fn get_key(chunk: &Vec<FileLanguageModel>) -> String {
            chunk
                .iter()
                .map(|f| f.base_model().path().to_string())
                .collect::<Vec<_>>()
                .join(",")
        }

        for file in &language_files {
            let formatted = file.format_batch(mode);
            let file_size = serde_json::to_string(&formatted)
                .map(|s| s.len())
                .unwrap_or(0);

            if file_size > max_chunk_chars {
                if !chunk.is_empty() {
                    let text = DefaultEnrich::build_prompt(&formatted_chunk, mode)?;
                    vibe_prompts.push(Prompt::new_with_key(text, get_key(&chunk)));
                    chunk = Vec::new();
                    formatted_chunk = Vec::new();
                    chunk_chars = 0;
                }
                let text = DefaultEnrich::build_prompt(&[formatted.clone()], mode)?;
                let key = file.base_model().path().to_string();
                vibe_prompts.push(Prompt::new_with_key(text, key));
                continue;
            }

            if chunk_chars + file_size > max_chunk_chars && !chunk.is_empty() {
                let text = DefaultEnrich::build_prompt(&formatted_chunk, mode)?;
                vibe_prompts.push(Prompt::new_with_key(text, get_key(&chunk)));
                chunk = Vec::new();
                formatted_chunk = Vec::new();
                chunk_chars = 0;
            }

            chunk_chars += file_size;
            chunk.push(file.clone());
            formatted_chunk.push(formatted);
        }

        if !chunk.is_empty() {
            let text = DefaultEnrich::build_prompt(&formatted_chunk, mode)?;
            vibe_prompts.push(Prompt::new_with_key(text, get_key(&chunk)));
        }
    } else {
        for file in &language_files {
            // Debt mode is code-only, skip markdown
            if mode == AnalyzeMode::Debt && file.node_language == Some(Language::Markdown) {
                continue;
            }
            let formatted = file.format_batch(mode);
            let text = DefaultEnrich::build_prompt(&[formatted], mode)?;
            let key = file.base_model().path().to_string();
            vibe_prompts.push(Prompt::new_with_key(text, key));
        }
    }

    // Execute batch via cluster
    let results = cluster
        .batch_call(
            &vibe_prompts,
            move |_result, done_prompts, total_prompts| {
                let percent = (done_prompts as f64 / total_prompts as f64 * 100.0) as usize;
                on_progress(mode, percent, done_prompts, total_prompts);
            },
        )
        .await?;

    // Parse results — match by key and collect stats
    let mut stats: Vec<WorkerStats> = Vec::new();
    let mut enriched_files: Vec<FileLanguageModel> = Vec::new();
    for result in &results {
        if let (Some(text), Some(key)) = (&result.text, &result.prompt.key) {
            let paths: Vec<&str> = key.split(',').collect();

            // Match by key
            let matching: Vec<FileLanguageModel> = language_files
                .iter()
                .filter(|f| paths.iter().any(|p| *p == f.base_model().path()))
                .cloned()
                .collect();

            if !matching.is_empty() {
                if let Ok(json) = serde_json::from_str::<Value>(text) {
                    match DefaultEnrich::parse_response(&matching, &json, mode) {
                        Ok(files) => enriched_files.extend(files),
                        Err(e) => println!("Parse error: {}", e),
                    }
                }
            }

            // Merge stats: find existing entry or push new
            if let Some(existing) = stats
                .iter_mut()
                .find(|s| s.host == result.host && s.model == result.model)
            {
                existing.files_processed += paths.len();
            } else {
                stats.push(WorkerStats {
                    host: result.host.clone(),
                    model: result.model.clone(),
                    provider: format!("{:?}", result.provider),
                    files_processed: paths.len(),
                });
            }
        }
    }

    let enriched_map: HashMap<String, &FileLanguageModel> = enriched_files
        .iter()
        .map(|f| (f.base_model().path().to_string(), f))
        .collect();

    let mut result = project.clone();
    result.summary = summary;

    for file in &mut result.files {
        if let FileModels::Language(lang) = file {
            if let Some(enriched) = enriched_map.get(lang.base_model().path()) {
                lang.analyze.merge(&enriched.analyze);
            }
        }
    }

    // Sort stats: most files first, then by host, then by model
    stats.sort_by(|a, b| {
        b.files_processed
            .cmp(&a.files_processed)
            .then(a.host.cmp(&b.host))
            .then(a.model.cmp(&b.model))
    });

    // Report stats
    on_done(stats);

    // Return enriched project
    Ok(result)
}
