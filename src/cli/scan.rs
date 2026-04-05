//! Scan command implementation for the vibe-analyzer CLI
//!
//! Handles scanning of single or multiple projects, collecting analysis data,
//! and exporting results to JSON format.

use anyhow::Result;
use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::print_info;
use crate::print_progress;
use crate::print_success;
use crate::print_warning;
use crate::scanner::export::export_project;
use crate::scanner::scanner;
use crate::utils::path::convert_relative;

/// Options for the scan command
pub struct ScanOptions {
    /// Optional custom path to scan instead of configured projects
    pub custom_path: Option<PathBuf>,
}

/// Executes project scanning and indexing
///
/// # Arguments
/// * `options` - ScanOptions containing optional custom path
///
/// # Returns
/// * `Ok(())` on success
/// * `Err` if no projects are configured or scanning fails
pub async fn exec(options: ScanOptions) -> Result<()> {
    let config = AppConfig::load()?;

    // Resolve projects to scan (custom path or from config)
    let projects = match options.custom_path {
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

        // Perform the actual scanning with progress and warning callbacks
        match scanner::scan_project(
            &project,
            |percent, current, total| {
                print_progress!("Processing... {:.1}% ({}/{})", percent, current, total)
            },
            |message| print_warning!("{}", message),
        )
        .await
        {
            Ok(analysis) => {
                all_analyses.push(analysis);
            }
            Err(e) => {
                print_warning!("Failed to index {}: {}", name, e);
            }
        }
    }

    // Export all projects to a single JSON file
    if !all_analyses.is_empty() {
        let json_path = export_project(all_analyses)?;
        println!();
        print_success!("Exported all projects to: {}", json_path.display());
    }

    Ok(())
}
