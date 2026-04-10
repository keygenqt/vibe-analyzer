//! Scan commands for extracting structure from knowledge sources.
//! Provides AST extraction, LLM enrichment, and OpenSearch indexing.

use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use crate::analyzer::analyzer;
use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::ollama_cluster;
use crate::configs::app::AppConfig;
use crate::exit_error;
use crate::export;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::print_error;
use crate::print_info;
use crate::print_progress;
use crate::print_success;
use crate::print_warning;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::scanner;

/// Scan actions for different analysis types.
#[derive(Subcommand)]
pub enum ScanAction {
    /// Extract AST structure from code.
    Ast {
        /// Target source (all if not specified)
        #[arg(long)]
        target: Option<String>,

        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,

        /// Output file path
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },

    /// Extract structure and enrich with LLM.
    Analyze {
        /// Target source (all if not specified)
        #[arg(long)]
        target: Option<String>,

        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,

        /// Output file path
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },

    /// Index scan results to OpenSearch.
    Index {
        /// Target source (all if not specified)
        #[arg(long)]
        target: Option<String>,
    },
}

/// Execute scan commands.
#[allow(unused_variables)]
pub async fn run(action: ScanAction) {
    let start_time = std::time::Instant::now();
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => exit_error!("{}", e),
    };
    match action {
        ScanAction::Ast {
            target,
            format,
            output,
        } => {
            let result = match scan_ast(&config, target).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };

            if !result.is_empty() {
                let options = ExportOptions::new(format, output);
                match export::writer::export_ast(result, &options) {
                    Ok(json_path) => {
                        let elapsed = start_time.elapsed();
                        print_success!(
                            "Exported to: {} (took {:.1}s)",
                            json_path.display(),
                            elapsed.as_secs_f64()
                        );
                    }
                    Err(e) => exit_error!("{}", e),
                };
            } else {
                print_warning!("No files to save");
            }
        }
        ScanAction::Analyze {
            target,
            format,
            output,
        } => {
            let result = match scan_enriches(&config, target).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };

            if !result.is_empty() {
                let options = ExportOptions::new(format, output);
                match export::writer::export_enriched(result, &options) {
                    Ok(json_path) => {
                        let elapsed = start_time.elapsed();
                        print_success!(
                            "Exported to: {} (took {:.1}s)",
                            json_path.display(),
                            elapsed.as_secs_f64()
                        );
                    }
                    Err(e) => exit_error!("{}", e),
                };
            } else {
                print_warning!("No files to save");
            }
        }
        ScanAction::Index { target } => {
            print_warning!("Coming soon: scan index");
        }
    }
}

/// Scans configured sources and extracts AST with metadata.
pub async fn scan_ast(config: &AppConfig, target: Option<String>) -> Result<Vec<SourceAnalysis>> {
    // Resolve sources to scan (custom path or from config)
    let sources = match target {
        Some(tg) => vec![config.get_source(&tg)?],
        None => config.get_sources(),
    };

    // Collect analysis results from all sources
    let mut all_analyses = Vec::new();

    for source in sources {
        let name = source.file_name().unwrap().to_string_lossy();
        print_info!("Start scan source: {}", name);
        match scanner::scan_source(
            &source,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total);
                if total == current {
                    println!()
                }
            },
            |message| print_warning!("{}", message),
        )
        .await
        {
            Ok(analysis) => {
                all_analyses.push(analysis);
            }
            Err(e) => {
                print_error!("Failed to scan {}: {}", name, e);
            }
        }
    }

    Ok(all_analyses)
}

/// Scans and enriches sources with LLM-powered analysis.
pub async fn scan_enriches(
    config: &AppConfig,
    target: Option<String>,
) -> Result<Vec<EnrichedSourceAnalysis>> {
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

    // Phase 1: Perform standard source scanning
    let all_analyses = scan_ast(config, target).await?;

    // Phase 2: Enrich each source with LLM analysis
    let mut enriched_all_analyses = Vec::new();

    for source_analyses in all_analyses {
        print_info!("Start scan enriched analysis");

        match analyzer::analyze_source(
            &source_analyses,
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
                print_error!("Failed to enrich {}: {}", source_analyses.source, e);
            }
        }
    }

    Ok(enriched_all_analyses)
}
