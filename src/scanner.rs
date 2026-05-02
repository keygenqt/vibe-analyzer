//! Core scanning functionality for codebase analysis.
//! Coordinates file discovery, AST parsing, license detection, and statistics aggregation.

use anyhow::Result;
use colored::Colorize;
use vibe_ast::{FileType, detect_file_type};

use crate::configs::app::AppConfig;
use crate::models::{file_models::FileModels, project::ProjectModel};
use crate::print_info;
use crate::print_newline;
use crate::print_progress;
use crate::print_warning;
use crate::utils::path;

/// Scans configured sources and extracts AST with metadata.
pub async fn scan_ast(config: &AppConfig, target: Option<String>) -> Result<Vec<ProjectModel>> {
    let sources = match target {
        Some(tg) => vec![config.get_source(&tg)?],
        None => config.get_sources(),
    };
    let mut projects = Vec::new();
    for source in sources {
        let name = source.file_name().unwrap().to_string_lossy();
        print_info!("Start scan source: {}", name.bold().blue());
        let canonical_path = dunce::canonicalize(&source)?;
        let files_full = path::source_files(&canonical_path)?;
        let total = files_full.len();
        let mut results = Vec::with_capacity(total);
        for (i, path) in files_full.iter().enumerate() {
            let percent = (i + 1) as f64 / total as f64 * 100.0;
            print_progress!("Processing... {:.1}% ({}/{})", percent, i + 1, total);

            let model = match detect_file_type(path) {
                FileType::Binary => FileModels::new_binary(path),
                FileType::Source(_) => match vibe_ast::parse_file(path) {
                    Ok(nodes) => FileModels::new_language(path, nodes),
                    Err(e) => {
                        print_warning!("Failed to parse {}: {}", path.display(), e);
                        continue;
                    }
                },
                FileType::Text => FileModels::new_text(path),
            };
            results.push(model);
        }
        print_newline!();
        projects.push(ProjectModel::new(&source, results));
    }
    Ok(projects)
}
