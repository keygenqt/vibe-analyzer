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
use crate::indexer::scan_index;
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
            let projects = match scan_ast(&config, target, true).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };
            if !projects.is_empty() {
                print_info!("Saving file...");
                let options = ExportOptions::new(format, output);
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
            let projects = match scan_analyze(&config, target, true).await {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };
            if !projects.is_empty() {
                print_info!("Saving file...");
                let options = ExportOptions::new(format, output);
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
            match scan_index(&config, target, true /* @todo */).await {
                Ok(result) => {
                    let elapsed = start_time.elapsed();
                    if result.index_files > 0 {
                        print_success!(
                            "Indexing completed. Files: {} (took {:.1}s)",
                            result.index_files,
                            elapsed.as_secs_f64()
                        );
                    } else {
                        print_success!("Index is up to date — all files are already in sync");
                    }
                }
                Err(e) => exit_error!("{}", e),
            };
        }
    }
}
