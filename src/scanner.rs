//! Core scanning functionality for codebase analysis.
//! Coordinates file discovery, AST parsing, license detection, and statistics aggregation.

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use vibe_ast::FileType;
use vibe_ast::detect_file_type;

use crate::configs::app::AppConfig;
use crate::models::file_models::FileModels;
use crate::models::project::ProjectModel;
use crate::print_info;
use crate::print_newline;
use crate::print_progress;
use crate::print_warning;
use crate::utils::constants::MAX_AST_FILE_SIZE;

/// Scans configured sources and extracts AST with metadata.
pub async fn scan_ast(
    config: &AppConfig,
    target: Option<String>,
    path: Option<PathBuf>,
    force: bool,
) -> Result<Vec<ProjectModel>> {
    let sources = if let Some(p) = path {
        vec![p]
    } else {
        match target {
            Some(tg) => vec![config.get_source(&tg)?],
            None => config.get_sources(),
        }
    };

    let mut projects = Vec::new();
    for source in sources {
        let name = source.file_name().unwrap().to_string_lossy();

        print_info!("Start scan source: {}", name.bold().blue());

        let canonical_path = dunce::canonicalize(&source)?;
        let files_full = vibe_fs::scan(&canonical_path, force)?;
        let total = files_full.len();

        if total == 0 {
            print_info!("No files to scan");
            return Ok(Vec::new());
        }

        let mut results = Vec::with_capacity(total);

        for (i, path) in files_full.iter().enumerate() {
            let percent = (i + 1) as f64 / total as f64 * 100.0;
            print_progress!("Processing... {:.1}% ({}/{})", percent, i + 1, total);
            let model = match detect_file_type(path) {
                FileType::Binary => FileModels::new_binary(path),
                FileType::Source(_) => {
                    // Skip files larger than MAX_AST_FILE_SIZE (10 MB)
                    if path.metadata().map(|m| m.len()).unwrap_or(0) > MAX_AST_FILE_SIZE {
                        FileModels::new_text(path)
                    } else {
                        match vibe_ast::parse_file(path) {
                            Ok(nodes) => FileModels::new_language(path, nodes),
                            Err(e) => {
                                print_warning!("Failed to parse {}: {}", path.display(), e);
                                continue;
                            }
                        }
                    }
                }
                FileType::Text => FileModels::new_text(path),
            };
            results.push(model);
        }
        print_newline!();
        projects.push(ProjectModel::new(&source, results));
    }
    Ok(projects)
}
