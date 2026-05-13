//! Analyze commands for extracting structure from sources.
//! Provides AST extraction, LLM enrichment, and OpenSearch indexing.

use clap::Subcommand;
use std::path::PathBuf;

use crate::analyzer::AnalyzeMode;
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
use crate::utils::path;

/// Scan, enrich, export, and index sources.
#[derive(Subcommand)]
pub enum AnalyzeAction {
    /// Index scan results to OpenSearch.
    Index {
        /// Target source (all if not specified)
        #[arg(long, short = 't')]
        target: Option<String>,

        /// Fast mode: skip LLM enrichment, index structure only
        #[arg(long, default_value_t = false)]
        fast: bool,

        /// Force reindex (ignore content hash)
        #[arg(long, default_value_t = false)]
        force: bool,
    },

    /// Export analyze results to file.
    Export {
        /// Analysis mode
        #[arg(long, short = 'm', value_delimiter = ',')]
        mode: Option<Vec<AnalyzeMode>>,

        /// Target source (all if not specified)
        #[arg(long, short = 't')]
        target: Option<String>,

        /// Export format
        #[arg(long, short = 'f')]
        format: Option<ExportFormat>,

        /// Source path
        #[arg(long, short = 'p')]
        path: Option<PathBuf>,

        /// Output file path
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },
}

/// Execute scan commands.
#[allow(unused_variables)]
pub async fn run(action: AnalyzeAction) {
    let start_time = std::time::Instant::now();
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => exit_error!("{}", e),
    };
    match action {
        AnalyzeAction::Index {
            target,
            fast,
            force,
        } => {
            match scan_index(&config, target, fast, force).await {
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
        AnalyzeAction::Export {
            mode,
            target,
            format,
            path,
            output,
        } => {
            let output = match path::resolve_file_path(output) {
                Ok(o) => o,
                Err(e) => exit_error!("{}", e),
            };
            let path = match path::resolve_dir_path(path) {
                Ok(o) => o,
                Err(e) => exit_error!("{}", e),
            };
            let modes = match mode.clone() {
                None => config.analyze.include.list(),
                Some(modes) => modes,
            };
            let projects = if modes == vec![AnalyzeMode::Ast] {
                scan_ast(&config, target, path, true).await
            } else {
                scan_analyze(&config, target, path, &modes, true).await
            };
            let projects = match projects {
                Ok(result) => result,
                Err(e) => exit_error!("{}", e),
            };
            if !projects.is_empty() {
                print_info!("Saving file...");
                let options = ExportOptions::new(format, output);
                let export_model = if modes == vec![AnalyzeMode::Ast] {
                    ExportModel::new_ast(&config.export, projects)
                } else {
                    ExportModel::new_analyze(&config.export, projects)
                };
                match writer::export(export_model, &options, &config.export) {
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
    }
}
