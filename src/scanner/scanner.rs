use anyhow::Result;
use chrono::Local;
use std::path::Path;

use crate::{
    scanner::{
        ast::parser::parse_ast,
        data::{
            file_analysis::FileAnalysis, project_analysis::ProjectAnalysis,
            project_stats::ProjectStats,
        },
    },
    utils::{self, constants, hash, path},
};

pub async fn scan_project(
    project_path: &Path,
    on_progress: fn(usize, usize, usize),
    on_warning: fn(String),
) -> Result<ProjectAnalysis> {
    let canonical_path = dunce::canonicalize(project_path)?;

    let project_display = if canonical_path.is_file() {
        canonical_path
            .parent()
            .unwrap_or(&canonical_path)
            .to_path_buf()
    } else {
        canonical_path.clone()
    };

    let files = path::project_files(&canonical_path)?;
    let total = files.len();

    let mut analysis = ProjectAnalysis {
        project: project_display.to_string_lossy().to_string(),
        indexed_at: Local::now().to_rfc3339(),
        stats: ProjectStats::default(),
        files: Vec::with_capacity(total),
    };

    for (i, file_info) in files.iter().enumerate() {
        let percent = (i + 1) as f64 / total as f64 * 100.0;
        on_progress(percent as usize, i + 1, total);

        let mime = utils::mime::detect_mime(&file_info.path)?;
        let mime_type = mime.to_string();
        let text_file = utils::mime::is_text(&mime, &file_info.path);

        let hash = hash::from_file(&file_info.path)?;

        let extension = path::get_extension(&file_info.path);
        let language = if text_file && !extension.is_empty() {
            Some(extension.clone())
        } else {
            None
        };

        let ast = if text_file && file_info.size < constants::MAX_AST_FILE_SIZE {
            match std::fs::read_to_string(&file_info.path) {
                Ok(content) => match parse_ast(&content, &extension) {
                    Ok(ast_data) => Some(ast_data),
                    Err(e) => {
                        on_warning(format!(
                            "Failed to parse AST for {}: {}",
                            file_info.path.display(),
                            e
                        ));
                        None
                    }
                },
                Err(e) => {
                    on_warning(format!(
                        "Failed to read file {}: {}",
                        file_info.path.display(),
                        e
                    ));
                    None
                }
            }
        } else {
            None
        };

        analysis.stats.update(ast.as_ref());

        analysis.files.push(FileAnalysis {
            path: file_info.relative_path.to_string_lossy().to_string(),
            mime_type,
            language,
            size_bytes: file_info.size,
            content_hash: hash,
            is_text: text_file,
            ast,
        });
    }

    Ok(analysis)
}
