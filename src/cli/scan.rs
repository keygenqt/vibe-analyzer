//! Scan commands for extracting structure from knowledge sources.
//! Provides AST extraction, LLM enrichment, and OpenSearch indexing.

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::analyzer::analyzer;
use crate::analyzer::data::output::enriched_source_analysis::EnrichedSourceAnalysis;
use crate::analyzer::ollama_cluster;
use crate::configs::app::AppConfig;
use crate::configs::app::IndexType;
use crate::exit_error;
use crate::export;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::opensearch::client::OpenSearchClient;
use crate::opensearch::data::requests::source_file::SourceFile;
use crate::opensearch::data::requests::source_file_analysis::SourceFileAnalysis;
use crate::opensearch::data::requests::source_meta::SourceMeta;
use crate::opensearch::data::responses::search_file_hash::SearchFileHashSource;
use crate::opensearch::indexer::IndexFilesReport;
use crate::opensearch::indexer::IndexMetaReport;
use crate::opensearch::indexer::IndexReport;
use crate::opensearch::indexer::Indexer;
use crate::opensearch::search::SearchClient;
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

        /// Force reindex (ignore content hash)
        #[arg(long, default_value_t = false)]
        force: bool,
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
            let result = match scan_ast(&config, target, None).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };

            if !result.is_empty() {
                print_info!("Saving file...");
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
            let result = match scan_enriches(&config, target, None).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };

            if !result.is_empty() {
                print_info!("Saving file...");
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
        ScanAction::Index { target, force } => {
            let result = match index_enriches(&config, target, force).await {
                Ok(result) => {
                    if result.files.files_indexed > 0 {
                        let elapsed = start_time.elapsed();
                        print_success!(
                            "Indexing completed. Sources: {}, Files: {}, Analysis: {} (took {:.1}s)",
                            result.meta.indexed,
                            result.files.files_indexed,
                            result.files.analysis_indexed,
                            elapsed.as_secs_f64()
                        );
                    } else {
                        print_success!(
                            "Index is up to date — all files are already indexed and database is in sync"
                        );
                    }
                }
                Err(e) => exit_error!("{}", e),
            };
        }
    }
}

/// Scans configured sources and extracts AST with metadata.
pub async fn scan_ast(
    config: &AppConfig,
    target: Option<String>,
    hashes: Option<SearchFileHashSource>,
) -> Result<Vec<SourceAnalysis>> {
    // Resolve sources to scan (custom path or from config)
    let sources = match target {
        Some(tg) => vec![config.get_source(&tg)?],
        None => config.get_sources(),
    };

    // Collect analysis results from all sources
    let mut all_analyses = Vec::new();

    for source in sources {
        let name = source.file_name().unwrap().to_string_lossy();
        print_info!("Start scan source: {}", name.bold().blue());
        match scanner::scan_source(
            &source,
            &hashes,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total);
                if total == current {
                    println!();
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
                println!();
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
    hashes: Option<SearchFileHashSource>,
) -> Result<Vec<EnrichedSourceAnalysis>> {
    let cluster = ollama_cluster::OllamaCluster::new(&config.ollama);
    // Validate Ollama configuration
    if cluster.is_empty() {
        anyhow::bail!("Ollama hosts not configured. Add it to ~/.vibe-analyzer/config.json");
    }

    // Check Ollama availability
    print_info!("Checking Ollama cluster...");
    match cluster
        .is_available(|current, total| {
            print_progress!("Checking availability {}/{}", current, total);
            if total == current {
                println!();
            }
        })
        .await
    {
        Ok(_) => {}
        Err(_) => {
            println!();
            anyhow::bail!(
                "One or more Ollama servers are not available. Check config or start Ollama service"
            )
        }
    }

    // Warm Ollama model
    match cluster
        .ensure_ready(|current, total| {
            print_progress!("Warming up server {}/{}", current, total);
            if total == current {
                println!();
            }
        })
        .await
    {
        Ok(_) => print_success!("All {} Ollama servers ready", cluster.len()),
        Err(e) => {
            println!();
            Err(e)?
        }
    }

    // Perform standard source scanning
    let all_analyses = scan_ast(config, target, hashes).await?;
    let total_files: usize = all_analyses.iter().map(|a| a.files.len()).sum();
    if total_files == 0 {
        print_info!("No files to scan");
        return Ok(Vec::new());
    }
    print_success!("AST scanning completed: {} source", all_analyses.len());

    // Enrich each source with LLM analysis
    let mut enriched_all_analyses = Vec::new();

    for source_analyses in all_analyses {
        let name = source_analyses.root.split("/").last().unwrap();
        print_info!("Start scan enriched analysis: {}", name.bold().blue());

        match analyzer::analyze_source(
            &source_analyses,
            &cluster,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total);
                if total == current {
                    println!();
                    print_info!("Cleaning up...");
                }
            },
            |message| print_info!("{}", message),
        )
        .await
        {
            Ok(analysis) => {
                enriched_all_analyses.push(analysis);
            }
            Err(e) => {
                println!();
                Err(e)?
            }
        }
    }

    Ok(enriched_all_analyses)
}

/// Index enriched analysis results to OpenSearch.
pub async fn index_enriches(
    config: &AppConfig,
    target: Option<String>,
    force: bool,
) -> Result<IndexReport> {
    // Connect to OpenSearch
    let client = OpenSearchClient::new(&config.opensearch.host)?;

    // Check connection
    if !client.healthcheck().await? {
        anyhow::bail!("OpenSearch is not available at {}", config.opensearch.host);
    }

    // Get hashes from OpenSearch for incremental indexing
    let search_client = SearchClient::new(client.clone());
    let hashes: Option<SearchFileHashSource> = if force {
        None
    } else {
        let targets: Vec<String> = match target.as_deref() {
            Some(tg) => vec![config.get_source(tg)?.to_string_lossy().to_string()],
            None => config
                .get_sources()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
        };
        let mut all_hashes = SearchFileHashSource {
            hashes: HashMap::new(),
        };
        for tg in targets {
            let index = config.get_index(&tg, IndexType::Files)?;
            let file_hashes = search_client.get_hashes(&index).await?;

            for h in file_hashes {
                all_hashes.hashes.insert(h.path, h.hash);
            }
        }
        Some(all_hashes)
    };

    // Create indexer coordinator
    let indexer = Indexer::new(client);

    // Get enriched data from analyzer (with incremental skip)
    let sources = scan_enriches(config, target, hashes).await?;

    // Check if there are files to index
    let total_files: usize = sources.iter().map(|s| s.files.len()).sum();
    if total_files == 0 {
        return Ok(IndexReport {
            meta: IndexMetaReport::default(),
            files: IndexFilesReport::default(),
        });
    }

    let total_files: usize = sources.iter().map(|s| s.files.len()).sum();
    print_success!(
        "LLM enrichment completed: {} sources, {} files",
        sources.len(),
        total_files
    );

    // Index metadata for all sources
    print_info!("Starting data indexing to OpenSearch...");
    let meta_items = SourceMeta::from_enriched(&sources);
    let report_meta = indexer.index_meta(&meta_items, force).await?;

    // Index files and analysis for each source
    let mut total_files = 0;
    let mut total_analysis = 0;

    for source in &sources {
        let root = PathBuf::from(&source.root);

        let file_items = SourceFile::from_enriched(source)?;
        let analysis_items = SourceFileAnalysis::from_enriched(source);

        let report_files = indexer
            .index_files(&root, &file_items, &analysis_items, force)
            .await?;

        total_files += report_files.files_indexed;
        total_analysis += report_files.analysis_indexed;
    }

    Ok(IndexReport {
        meta: report_meta,
        files: IndexFilesReport {
            files_indexed: total_files,
            analysis_indexed: total_analysis,
        },
    })
}
