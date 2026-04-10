//! Core scanning functionality for codebase analysis.
//! Coordinates file discovery, AST parsing, license detection, and statistics aggregation.

use anyhow::Result;
use chrono::Local;
use std::path::Path;

use crate::scanner::ast::parser::parse_ast;
use crate::scanner::data::output::file_analysis::FileAnalysis;
use crate::scanner::data::output::source_analysis::SourceAnalysis;
use crate::scanner::data::output::source_stats::SourceStats;
use crate::scanner::detectors;
use crate::utils::constants;
use crate::utils::hash;
use crate::utils::path;
use crate::utils::{self};

/// Scans a source and returns complete analysis results
pub async fn scan_source(
    source_path: &Path,
    on_progress: fn(usize, usize, usize),
    on_warning: fn(String),
) -> Result<SourceAnalysis> {
    let canonical_path = dunce::canonicalize(source_path)?;

    let source_display = if canonical_path.is_file() {
        canonical_path
            .parent()
            .unwrap_or(&canonical_path)
            .to_path_buf()
    } else {
        canonical_path.clone()
    };

    let files = path::source_files(&canonical_path)?;
    let total = files.len();
    let readme = detectors::readme::detect_readme(&files);
    let license = detectors::license::detect_license(&files).await;

    let mut analysis = SourceAnalysis {
        source: source_display.to_string_lossy().to_string(),
        indexed_at: Local::now().to_rfc3339(),
        readme,
        license,
        stats: SourceStats::default(),
        files: Vec::with_capacity(total),
    };

    for (i, file_info) in files.iter().enumerate() {
        let percent = (i + 1) as f64 / total as f64 * 100.0;
        on_progress(percent as usize, i + 1, total);

        let mime = utils::mime::detect_mime(&file_info.path)?;
        let text_file = utils::mime::is_text(&mime, &file_info.path);

        let file_id = hash::file_id_from_file(&file_info.path)?;
        let extension = path::get_extension(&file_info.path);

        let (lines_of_code, size_chars) = if text_file {
            match std::fs::read_to_string(&file_info.path) {
                Ok(content) => (Some(content.lines().count()), Some(content.chars().count())),
                Err(e) => {
                    on_warning(format!(
                        "Failed to read file {}: {}",
                        file_info.path.display(),
                        e
                    ));
                    (None, None)
                }
            }
        } else {
            (None, None)
        };

        if let Some(loc) = lines_of_code {
            analysis.stats.add_lines_of_code(loc);
        }
        if let Some(chars) = size_chars {
            analysis.stats.add_chars(chars);
        }

        let ast = if let Some(ext) = &extension {
            if text_file && file_info.size < constants::MAX_AST_FILE_SIZE {
                match std::fs::read_to_string(&file_info.path) {
                    Ok(content) => match parse_ast(&content, ext) {
                        Ok(ast_data) => ast_data,
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
            }
        } else {
            None
        };

        analysis.stats.update(ast.as_ref());
        analysis.stats.add_size_bytes(file_info.size);

        analysis.files.push(FileAnalysis {
            path: file_info.relative_path.to_string_lossy().to_string(),
            extension,
            size_bytes: file_info.size,
            file_id,
            is_text: text_file,
            lines_of_code,
            size_chars,
            ast,
        });
    }

    Ok(analysis)
}
