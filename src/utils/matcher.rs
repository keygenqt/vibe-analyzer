use std::path::Path;

use crate::utils::constants::{DEFAULT_IGNORE_DIRS, DEFAULT_IGNORE_FILES};

/// Checks if file/dir matches default ignore patterns (build artifacts, system files, logs)
pub fn is_ignore(path: &Path) -> bool {
    if path.is_dir() {
        for ignore_dir in DEFAULT_IGNORE_DIRS {
            if path.components().any(|c| c.as_os_str() == *ignore_dir) {
                return true;
            }
        }
        return false;
    }
    if path.is_file() {
        for ignore_file in DEFAULT_IGNORE_FILES {
            if ignore_file.contains("*") {
                if path
                    .extension()
                    .map(|os| os.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "".to_string())
                    == ignore_file.trim_start_matches("*.")
                {
                    return true;
                }
            } else {
                if path.components().any(|c| c.as_os_str() == *ignore_file) {
                    return true;
                }
            }
        }
        return false;
    }
    true
}
