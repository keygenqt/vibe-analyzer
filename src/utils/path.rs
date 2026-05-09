//! Application path utilities.
//! Provides config and data directory paths.

use anyhow::Result;
use dirs::home_dir;
use std::path::PathBuf;

use crate::utils::constants::CONFIG_DIR_NAME;

/// Get config directory path: ~/.vibe-analyzer/
pub fn config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}

/// Resolve and validate output file path.
/// Checks that parent directory exists, returns resolved absolute path.
pub fn resolve_file_path(path: Option<PathBuf>) -> Result<Option<PathBuf>> {
    match path {
        Some(ref path) => {
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or_else(|| std::path::Path::new("."));
            match vibe_fs::resolve(parent) {
                Ok(resolved) if resolved.exists() => {
                    Ok(Some(resolved.join(path.file_name().unwrap())))
                }
                Ok(resolved) => Err(anyhow::anyhow!(
                    "Output directory does not exist: {}",
                    resolved.display()
                )),
                Err(e) => Err(anyhow::anyhow!("Failed to resolve path: {}", e)),
            }
        }
        None => Ok(None),
    }
}

/// Resolve and validate output directory path.
/// Checks that directory exists, returns resolved absolute path.
pub fn resolve_dir_path(path: Option<PathBuf>) -> Result<Option<PathBuf>> {
    match path {
        Some(ref path) => match vibe_fs::resolve(path) {
            Ok(resolved) if resolved.exists() && resolved.is_dir() => Ok(Some(resolved)),
            Ok(resolved) if !resolved.exists() => Err(anyhow::anyhow!(
                "Directory does not exist: {}",
                resolved.display()
            )),
            Ok(resolved) => Err(anyhow::anyhow!(
                "Path is not a directory: {}",
                resolved.display()
            )),
            Err(e) => Err(anyhow::anyhow!("Failed to resolve path: {}", e)),
        },
        None => Ok(None),
    }
}
