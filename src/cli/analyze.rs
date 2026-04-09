//! Analyze command for LLM-powered code enrichment.
//! Scans projects and enhances results with descriptions and search tags.

use crate::analyzer;
use crate::analyzer::data::output::enriched_project_analysis::EnrichedProjectAnalysis;
use crate::analyzer::ollama_cluster;
use crate::cli::scan::ScanOptions;
use crate::cli::scan::{self};
use crate::configs::app::AppConfig;
use crate::export;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::print_error;
use crate::print_info;
use crate::print_progress;
use crate::print_success;
use crate::print_warning;
use anyhow::Result;
use std::path::PathBuf;

/// Options for the analyze command.
pub struct AnalyzeOptions {
    /// Optional custom path to scan instead of configured projects
    pub custom_path: Option<PathBuf>,
    /// Export format (json, json5, toon, xml)
    pub format: ExportFormat,
    /// Optional custom output file path
    pub output: Option<PathBuf>,
}

/// Executes project scanning with LLM enrichment.
pub async fn exec(options: AnalyzeOptions) -> Result<()> {
    let config = AppConfig::load()?;

    let start_time = std::time::Instant::now();
    let enriched_all_analyses = scan(&config, &options).await?;
    let elapsed = start_time.elapsed();

    // Export all projects to a single JSON file
    if !enriched_all_analyses.is_empty() {
        let options = ExportOptions::new(options.format, options.output);
        let json_path = export::writer::export_enriched_projects(enriched_all_analyses, &options)?;
        print_success!(
            "Exported all projects to: {} (took {:.1}s)",
            json_path.display(),
            elapsed.as_secs_f64()
        );
    }

    Ok(())
}

/// Scans and enriches projects with LLM-powered analysis.
pub async fn scan(
    config: &AppConfig,
    options: &AnalyzeOptions,
) -> Result<Vec<EnrichedProjectAnalysis>> {
    let cluster = ollama_cluster::OllamaCluster::new(&config.ollama);
    // Validate Ollama configuration
    if cluster.is_empty() {
        anyhow::bail!("Ollama hosts not configured. Add it to ~/.vibe-analyzer/config.json");
    }

    // Check Ollama availability and warm up model
    print_info!("Checking Ollama cluster...");
    if !cluster
        .is_available(|current, total| {
            print_progress!("Checking availability {}/{}", current, total);
            if total == current {
                println!()
            }
        })
        .await?
    {
        anyhow::bail!(
            "One or more Ollama servers are not available. Check config or start Ollama service"
        );
    }
    cluster
        .ensure_ready(|current, total| {
            print_progress!("Warming up server {}/{}", current, total);
            if total == current {
                println!()
            }
        })
        .await?;
    print_success!("All {} Ollama servers ready", cluster.len());

    // Phase 1: Perform standard project scanning
    let scan_options = ScanOptions {
        custom_path: options.custom_path.clone(),
        format: options.format,
        output: options.output.clone(),
    };
    let all_analyses = scan::scan(&config, &scan_options).await?;

    // Phase 2: Enrich each project with LLM analysis
    let mut enriched_all_analyses = Vec::new();

    for project_analyses in all_analyses {
        print_info!("Start scan enriched analysis");

        match analyzer::analyzer::analyze_project(
            &project_analyses,
            &cluster,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total);
                if total == current {
                    println!()
                }
            },
            |message| print_warning!("{}", message),
            |message| print_info!("{}", message),
        )
        .await
        {
            Ok(analysis) => {
                enriched_all_analyses.push(analysis);
            }
            Err(e) => {
                print_error!("Failed to index {}: {}", project_analyses.project, e);
            }
        }
    }

    Ok(enriched_all_analyses)
}
