//! Scan commands for extracting structure from knowledge sources.
//! Provides AST extraction, LLM enrichment, and OpenSearch indexing.

use clap::Subcommand;
use std::path::PathBuf;

use crate::analyzer::scan_analyze;
use crate::configs::app::AppConfig;
use crate::exit_error;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::export::writer;
use crate::export::writer::ExportModel;
use crate::print_info;
use crate::print_success;
use crate::print_warning;
use crate::scanner::scan_ast;

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
            let projects = match scan_ast(&config, target).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };
            if !projects.is_empty() {
                print_info!("Saving file...");
                let options = ExportOptions::new("va", format, output);
                match writer::export(ExportModel::new_ast(projects), &options) {
                    Ok(path) => {
                        let elapsed = start_time.elapsed();
                        print_success!(
                            "Exported to: {} (took {:.1}s)",
                            path.display(),
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
            let projects = match scan_analyze(&config, target).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };
            if !projects.is_empty() {
                print_info!("Saving file...");
                let options = ExportOptions::new("va", format, output);
                match writer::export(ExportModel::new_analyze(projects), &options) {
                    Ok(path) => {
                        let elapsed = start_time.elapsed();
                        print_success!(
                            "Exported to: {} (took {:.1}s)",
                            path.display(),
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
            // let result = match index_enriches(&config, target, force).await {
            //     Ok(result) => {
            //         if result.files.files_indexed > 0 {
            //             let elapsed = start_time.elapsed();
            //             print_success!(
            //                 "Indexing completed. Sources: {}, Files: {}, Analysis: {} (took {:.1}s)",
            //                 result.meta.indexed,
            //                 result.files.files_indexed,
            //                 result.files.analysis_indexed,
            //                 elapsed.as_secs_f64()
            //             );
            //         } else {
            //             print_success!(
            //                 "Index is up to date — all files are already indexed and database is in sync"
            //             );
            //         }
            //     }
            //     Err(e) => exit_error!("{}", e),
            // };
        }
    }
}

// /// Index enriched analysis results to OpenSearch.
// pub async fn index_enriches(
//     config: &AppConfig,
//     target: Option<String>,
//     force: bool,
// ) -> Result<IndexReport> {
//     // Connect to OpenSearch
//     let client = OpenSearchClient::new(&config.opensearch.host)?;

//     // Check connection
//     if !client.healthcheck().await? {
//         anyhow::bail!("OpenSearch is not available at {}", config.opensearch.host);
//     }

//     // Create indexer coordinator
//     let indexer = Indexer::new(client.clone());

//     // Cleanup orphaned data after successful indexing
//     print_info!("Cleaning up orphaned data...");
//     indexer.cleanup(config).await?;

//     // Get hashes from OpenSearch for incremental indexing
//     let search_client = SearchClient::new(client);
//     let hashes: Option<OpensearchListHashesResponse> = if force {
//         None
//     } else {
//         let targets: Vec<String> = match target.as_deref() {
//             Some(tg) => vec![config.get_source(tg)?.to_string_lossy().to_string()],
//             None => config
//                 .get_sources()
//                 .iter()
//                 .map(|s| s.to_string_lossy().to_string())
//                 .collect(),
//         };
//         let mut all_hashes = OpensearchListHashesResponse {
//             hashes: HashMap::new(),
//         };
//         for tg in targets {
//             let index = config.get_index(&tg, IndexType::Files)?;
//             let file_hashes = search_client.get_hashes(&index).await?;

//             for h in file_hashes {
//                 all_hashes.hashes.insert(h.path, h.hash);
//             }
//         }
//         Some(all_hashes)
//     };

//     // Get enriched data from analyzer (with incremental skip)
//     let sources = scan_enriches(config, target, hashes).await?;

//     // Check if there are files to index
//     let total_files: usize = sources.iter().map(|s| s.files.len()).sum();
//     if total_files == 0 {
//         return Ok(IndexReport {
//             meta: IndexMetaReport::default(),
//             files: IndexFilesReport::default(),
//         });
//     }

//     let total_files: usize = sources.iter().map(|s| s.files.len()).sum();
//     print_success!(
//         "LLM enrichment completed: {} sources, {} files",
//         sources.len(),
//         total_files
//     );

//     // Index metadata for all sources
//     print_info!("Starting data indexing to OpenSearch...");
//     let meta_items = OpensearchProjectMetaRequest::from_enriched(&sources);
//     let report_meta = indexer.index_meta(&meta_items, force).await?;

//     // Index files and analysis for each source
//     let mut total_files = 0;
//     let mut total_analysis = 0;

//     for source in &sources {
//         let root = PathBuf::from(&source.root);

//         let file_items = OpensearchFileContentRequest::from_enriched(source)?;
//         let analysis_items = OpensearchFileStatsRequest::from_enriched(source);

//         let report_files = indexer
//             .index_files(&root, &file_items, &analysis_items, force)
//             .await?;

//         total_files += report_files.files_indexed;
//         total_analysis += report_files.analysis_indexed;
//     }

//     Ok(IndexReport {
//         meta: report_meta,
//         files: IndexFilesReport {
//             files_indexed: total_files,
//             analysis_indexed: total_analysis,
//         },
//     })
// }
