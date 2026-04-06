//! JSON export functionality

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use crate::scanner::data::multi_project_analysis::MultiProjectAnalysis;
use crate::scanner::data::project_analysis::ProjectAnalysis;
use crate::utils::path;

/// Exports project analyses to a single JSON file in Downloads folder
pub fn export_project(projects: Vec<ProjectAnalysis>) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%Y%m%d_%H%M%S");
    let filename = format!("vibe-analyzer_{}.json", timestamp);

    let downloads_dir = path::get_downloads_dir();
    let filepath = downloads_dir.join(filename);

    let multi_analysis = MultiProjectAnalysis {
        indexed_at: Local::now().to_rfc3339(),
        projects,
    };

    let json_content = serde_json::to_string_pretty(&multi_analysis)?;
    fs::write(&filepath, json_content)?;

    Ok(filepath)
}
