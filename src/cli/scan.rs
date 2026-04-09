//! Scan command implementation for the vibe-analyzer CLI.
//! Handles project scanning, AST extraction, and JSON export to Downloads folder.

use anyhow::Result;
use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::export;
use crate::export::options::ExportFormat;
use crate::export::options::ExportOptions;
use crate::print_error;
use crate::print_info;
use crate::print_progress;
use crate::print_success;
use crate::print_warning;
use crate::scanner::data::output::project_analysis::ProjectAnalysis;
use crate::scanner::scanner;
use crate::utils::path::convert_relative;

/// Options for the scan command
pub struct ScanOptions {
    /// Optional custom path to scan instead of configured projects
    pub custom_path: Option<PathBuf>,

    /// Export format (json or toon)
    pub format: ExportFormat,

    /// Optional custom output file path
    /// If not provided, uses default path in Downloads directory
    pub output: Option<PathBuf>,
}

/// Executes project scanning and indexing
pub async fn exec(options: ScanOptions) -> Result<()> {
    let config = AppConfig::load()?;

    let start_time = std::time::Instant::now();
    let all_analyses = scan(&config, &options).await?;
    let elapsed = start_time.elapsed();

    // Export all projects to a single JSON file
    if !all_analyses.is_empty() {
        let options = ExportOptions::new(options.format, options.output);
        let json_path = export::writer::export_projects(all_analyses, &options)?;
        println!();
        print_success!(
            "Exported all projects to: {} (took {:.1}s)",
            json_path.display(),
            elapsed.as_secs_f64()
        );
    }

    Ok(())
}
/// Scans configured projects and extracts AST with metadata.
pub async fn scan(config: &AppConfig, options: &ScanOptions) -> Result<Vec<ProjectAnalysis>> {
    // Resolve projects to scan (custom path or from config)
    let projects = match options.custom_path.clone() {
        Some(path) => vec![convert_relative(&path.to_string_lossy()).unwrap_or(path)],
        None => {
            let projects = config.get_projects();
            if projects.is_empty() {
                anyhow::bail!("No projects configured. Use `vibe-analyzer add <path>` first.");
            }
            projects
        }
    };

    // Collect analysis results from all projects
    let mut all_analyses = Vec::new();

    for project in projects {
        let name = project.file_name().unwrap().to_string_lossy();
        print_info!("Start scan project: {}", name);

        match scanner::scan_project(
            &project,
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
                print_error!("Failed to index {}: {}", name, e);
            }
        }
    }

    Ok(all_analyses)
}
