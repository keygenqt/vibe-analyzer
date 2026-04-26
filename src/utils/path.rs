//! File system path utilities for the vibe-analyzer.
//! Provides path resolution, configuration directory handling, and source file discovery.

use anyhow::Result;
use chrono::Local;
use dirs::home_dir;
use ignore::WalkBuilder;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use crate::export::options::ExportFormat;
use crate::scanner::data::input::scan_file_info_input::ScanFileInfoInput;
use crate::utils::app;
use crate::utils::constants::CONFIG_DIR_NAME;
use crate::utils::matcher;

/// Returns file extension (lowercase)
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// Get config directory path: ~/.vibe-analyzer/
pub fn get_config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}

/// Get Downloads directory path
pub fn get_downloads_dir() -> PathBuf {
    dirs::download_dir().expect("Failed to get downloads directory")
}

/// Generates the default export file path in the Downloads directory.
pub fn get_default_export(format: ExportFormat) -> PathBuf {
    let timestamp = Local::now().format("%d-%m-%Y_%H-%M-%S");
    let filename = format!("{}-{}.{}", app::app_name(), timestamp, format.extension());
    let downloads_dir = get_downloads_dir();
    downloads_dir.join(filename)
}

/// Resolves a path to absolute form, expanding ~, ., ..
pub fn convert_relative(path: &str) -> Option<PathBuf> {
    let path = path.trim();
    if path.is_empty() {
        return None;
    }

    // Handle special cases
    if path == "." {
        return env::current_dir().ok();
    }
    if path == ".." {
        return env::current_dir()
            .ok()
            .and_then(|d| d.parent().map(|p| p.to_path_buf()));
    }

    // Expand ~/ to home directory
    let path = if path.starts_with("~/") {
        env::var("HOME")
            .ok()
            .map(|home| path.replacen("~/", &format!("{}/", home), 1))
            .unwrap_or_else(|| path.to_string())
    } else {
        path.to_string()
    };

    // Expand ./ and ../
    let path = if path.starts_with("./") {
        env::current_dir()
            .ok()
            .map(|cwd| path.replacen("./", &format!("{}/", cwd.display()), 1))
            .unwrap_or(path)
    } else if path.starts_with("../") {
        env::current_dir()
            .ok()
            .map(|cwd| format!("{}/{}", cwd.display(), path))
            .unwrap_or(path)
    } else {
        path
    };

    let path = PathBuf::from(&path);

    // Normalize and resolve symlinks
    if path.exists() {
        Some(dunce::canonicalize(&path).unwrap_or(path))
    } else if path.is_relative() {
        env::current_dir()
            .ok()
            .map(|cwd| dunce::canonicalize(cwd.join(&path)).unwrap_or(cwd.join(path)))
    } else {
        Some(path)
    }
}

/// Returns list of files in a source (supports both single file and directory)
pub fn source_files(source_path: &Path) -> Result<Vec<ScanFileInfoInput>> {
    let source_path_abs = dunce::canonicalize(source_path)?;

    // Handle single file case
    if source_path_abs.is_file() {
        let size = source_path_abs.metadata()?.len();
        let file_name = source_path_abs
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        return Ok(vec![ScanFileInfoInput {
            path: source_path_abs,
            relative_path: PathBuf::from(file_name),
            size,
        }]);
    }

    // Check for .gitignore presence
    let gitignore_path = source_path_abs.join(".gitignore");
    let has_gitignore = gitignore_path.exists();

    // Check for .gitmodules presence
    let source = source_path_abs.clone();
    let submodule_paths: Vec<PathBuf> = std::fs::read_to_string(source.join(".gitmodules"))
        .unwrap_or_default()
        .lines()
        .filter(|l| l.trim().starts_with("path = "))
        .filter_map(|l| l.trim().strip_prefix("path = "))
        .map(|p| source.join(p.trim()))
        .collect();

    // Scan directory recursively
    let mut files = Vec::new();
    let walker = WalkBuilder::new(&source_path_abs)
        .require_git(false)
        .git_ignore(true)
        .hidden(true)
        .git_global(false)
        .git_exclude(false)
        .follow_links(false)
        .filter_entry(move |entry| {
            !submodule_paths
                .iter()
                .any(|sm| entry.path() == sm || entry.path().starts_with(sm))
        })
        .build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Apply default ignore rules if no .gitignore
        if !has_gitignore && matcher::is_ignore(path) {
            continue;
        }

        let relative_path = path.strip_prefix(&source_path_abs)?.to_path_buf();
        let size = path.metadata()?.len();

        files.push(ScanFileInfoInput {
            path: path.to_path_buf(),
            relative_path,
            size,
        });
    }

    Ok(files)
}
