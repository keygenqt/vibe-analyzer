//! Analyze command for LLM-powered code enrichment.
//! Scans projects and enhances results with descriptions and search tags.

use crate::analyzer::data::output::enriched_project_analysis::EnrichedProjectAnalysis;
use crate::analyzer::ollama;
use crate::cli::scan::{self, ScanOptions};
use crate::configs::app::AppConfig;
use crate::export::options::{ExportFormat, ExportOptions};
use crate::{
    analyzer, export, print_error, print_info, print_progress, print_success, print_warning,
};
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
        println!();
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
    // Validate Ollama configuration
    if config.ollama.host.is_empty() {
        anyhow::bail!("Ollama host not configured. Add it to ~/.vibe-analyzer/config.json");
    }

    // Check Ollama availability and warm up model
    let ollama_client = ollama::OllamaClient::new(&config.ollama);
    if !ollama_client.is_available().await? {
        anyhow::bail!(
            "Ollama not available at {}. Check config or start Ollama service",
            config.ollama.host
        );
    }

    print_info!("Checking Ollama and warming up model...");
    if let Err(e) = ollama_client.ensure_ready().await {
        anyhow::bail!("{}", e);
    }
    print_success!("Ollama model '{}' ready", ollama_client.config.model);

    // Phase 1: Perform standard project scanning
    let scan_options = ScanOptions {
        custom_path: options.custom_path.clone(),
        format: options.format,
        output: options.output.clone(),
    };
    let all_analyses = scan::scan(&config, &scan_options).await?;
    println!();

    // Phase 2: Enrich each project with LLM analysis
    let mut enriched_all_analyses = Vec::new();

    for project_analyses in all_analyses {
        print_info!("Start scan enriched analysis");

        match analyzer::analyzer::analyze_project(
            &project_analyses,
            &ollama_client,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total)
            },
            |message| {
                println!();
                print_warning!("{}", message)
            },
        )
        .await
        {
            Ok(analysis) => {
                enriched_all_analyses.push(analysis);
            }
            Err(e) => {
                println!();
                print_error!("Failed to index {}: {}", project_analyses.project, e);
            }
        }
    }

    Ok(enriched_all_analyses)
}
