//! Application path utilities.
//! Provides config and data directory paths.

use dirs::home_dir;
use std::path::PathBuf;

use crate::utils::constants::CONFIG_DIR_NAME;

/// Get config directory path: ~/.vibe-analyzer/
pub fn config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}
