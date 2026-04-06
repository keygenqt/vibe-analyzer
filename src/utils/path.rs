//! File system path utilities

use anyhow::Result;
use dirs::home_dir;
use ignore::WalkBuilder;
use std::env;
use std::path::{Path, PathBuf};

use crate::scanner::data::file_info::FileInfo;
use crate::utils::constants::{CONFIG_DIR_NAME, CONFIG_FILE_NAME};
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

/// Get config file path: ~/.vibe-analyzer/config.json
pub fn get_config_path() -> PathBuf {
    get_config_dir().join(CONFIG_FILE_NAME)
}

/// Get Downloads directory path
pub fn get_downloads_dir() -> PathBuf {
    dirs::download_dir().expect("Failed to get downloads directory")
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

/// Returns list of files in a project (supports both single file and directory)
pub fn project_files(project_path: &Path) -> Result<Vec<FileInfo>> {
    let project_path_abs = dunce::canonicalize(project_path)?;

    // Handle single file case
    if project_path_abs.is_file() {
        let size = project_path_abs.metadata()?.len();
        let file_name = project_path_abs
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        return Ok(vec![FileInfo {
            path: project_path_abs,
            relative_path: PathBuf::from(file_name),
            size,
        }]);
    }

    // Check for .gitignore presence
    let gitignore_path = project_path_abs.join(".gitignore");
    let has_gitignore = gitignore_path.exists();

    // Scan directory recursively
    let mut files = Vec::new();
    let walker = WalkBuilder::new(&project_path_abs)
        .require_git(false)
        .git_ignore(true)
        .hidden(true)
        .git_global(false)
        .git_exclude(false)
        .follow_links(false)
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

        let relative_path = path.strip_prefix(&project_path_abs)?.to_path_buf();
        let size = path.metadata()?.len();

        files.push(FileInfo {
            path: path.to_path_buf(),
            relative_path,
            size,
        });
    }

    Ok(files)
}
