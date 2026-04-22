//! Core scanning functionality for codebase analysis.
//! Coordinates file discovery, AST parsing, license detection, and statistics aggregation.

use anyhow::Result;
use chrono::Local;
use std::path::Path;

use crate::opensearch::data::responses::opensearch_list_hashes_response::OpensearchListHashesResponse;
use crate::scanner::ast::parser::parse_ast;
use crate::scanner::data::input::scan_file_stats_input::ScanFileStatsInput;
use crate::scanner::data::output::scan_file_analysis_output::ScanFileAnalysisOutput;
use crate::scanner::data::output::scan_project_analysis_output::ScanProjectAnalysisOutput;
use crate::scanner::data::output::scan_project_stats_output::ScanProjectStatsOutput;
use crate::scanner::detectors;
use crate::utils::constants;
use crate::utils::hash;
use crate::utils::path;

/// Scans a source and returns complete analysis results
pub async fn scan_source(
    source_path: &Path,
    hashes: &Option<OpensearchListHashesResponse>,
    on_progress: fn(usize, usize, usize),
    on_warning: fn(String),
) -> Result<ScanProjectAnalysisOutput> {
    let canonical_path = dunce::canonicalize(source_path)?;
    let source_display = if canonical_path.is_file() {
        canonical_path
            .parent()
            .unwrap_or(&canonical_path)
            .to_path_buf()
    } else {
        canonical_path.clone()
    };

    // Get full stats about source
    let files_full = path::source_files(&canonical_path)?;
    let mut source_stats = ScanProjectStatsOutput::default();
    for file_info in files_full.iter() {
        source_stats.add_stats(&ScanFileStatsInput::new(&file_info)?);
    }

    // Files with filter unchanged
    let mut files = files_full.clone();
    if let Some(hash_source) = hashes {
        files.retain(|f| {
            let full_path = f.path.to_string_lossy().to_string();
            match hash_source.hashes.get(&full_path) {
                Some(indexed_hash) => {
                    hash::hash_from_content(&f.path).unwrap_or_default() != *indexed_hash
                }
                None => true,
            }
        });
    }

    let total = files.len();
    let readme = detectors::readme::detect_readme(&files);
    let license = detectors::license::detect_license(&files).await;

    let mut analysis = ScanProjectAnalysisOutput {
        root: source_display.to_string_lossy().to_string(),
        indexed_at: Local::now().to_rfc3339(),
        readme,
        license,
        stats: source_stats,
        files: Vec::with_capacity(total),
    };

    for (i, file_info) in files.iter().enumerate() {
        let percent = (i + 1) as f64 / total as f64 * 100.0;
        on_progress(percent as usize, i + 1, total);
        let stats = ScanFileStatsInput::new(&file_info)?;

        let ast = if let Some(ext) = &stats.extension {
            if stats.is_text && file_info.size < constants::MAX_AST_FILE_SIZE {
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
        analysis.files.push(ScanFileAnalysisOutput {
            path: file_info.relative_path.to_string_lossy().to_string(),
            extension: stats.extension,
            size_bytes: file_info.size,
            hash: stats.hash,
            is_text: stats.is_text,
            lines_of_code: if stats.is_text {
                Some(stats.lines_of_code)
            } else {
                None
            },
            size_chars: if stats.is_text {
                Some(stats.size_chars)
            } else {
                None
            },
            ast,
        });
    }

    Ok(analysis)
}
