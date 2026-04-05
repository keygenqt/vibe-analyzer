use askalono::{Store, TextData};
use std::fs::File;
use tokio::sync::OnceCell;

use crate::{
    configs::app::AppConfig,
    scanner::data::{file_info::FileInfo, license_info::LicenseInfo},
    utils::path::get_config_dir,
};

/// Global license store initialized once asynchronously
static LICENSE_STORE: OnceCell<Store> = OnceCell::const_new();

/// Returns a reference to the globally initialized license store
///
/// # Initialization logic
/// 1. If cache exists at `~/.vibe-analyzer/askalono-cache.bin.zstd` -> load from cache (fast, ~300ms)
/// 2. Otherwise -> download SPDX data, parse JSON files, create cache (slow on first run)
async fn get_license_store() -> &'static Store {
    LICENSE_STORE.get_or_init(init_license_store).await
}

/// Initializes the license store from cache or downloads SPDX data
async fn init_license_store() -> Store {
    let cache_path = get_config_dir().join("askalono-cache.bin.zstd");

    // Fast path: load pre-built binary cache
    if cache_path.exists() {
        let file = File::open(cache_path).expect("Failed to open cache file");
        Store::from_cache(file).expect("Failed to load cache")
    }
    // Slow path: first run - download and parse SPDX JSON files
    else {
        let config = AppConfig::load().expect("Failed to load config");
        config
            .spdx
            .ensure_data()
            .await
            .expect("Failed to setup SPDX data");

        let mut store = Store::new();
        let details_path = config.spdx.get_spdx_path();

        // Parse all SPDX JSON license files
        store
            .load_spdx(&details_path, false)
            .expect("Failed to load SPDX data");

        // Save cache for future runs
        let cache_file = File::create(cache_path).expect("Failed to create cache file");
        store.to_cache(cache_file).expect("Failed to save cache");

        store
    }
}

/// License detector using askalono library
pub struct LicenseDetector;

impl LicenseDetector {
    /// Searches for license files in the file list and detects license type
    ///
    /// # File name patterns recognized
    /// - license, LICENSE, License
    /// - copying, COPYING, Copying
    /// - licence, LICENCE, Licence
    pub async fn find_license(files: &[FileInfo]) -> Option<LicenseInfo> {
        let license_patterns = [
            "license", "LICENSE", "License", "copying", "COPYING", "Copying", "licence", "LICENCE",
            "Licence",
        ];

        for file in files {
            let stem = file.path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

            if license_patterns.contains(&stem) {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    if let Some((spdx_id, name, confidence)) =
                        Self::detect_from_content(&content).await
                    {
                        return Some(LicenseInfo {
                            spdx_id,
                            name,
                            confidence,
                            path: file.relative_path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }
        None
    }

    /// Detects license type by analyzing file content
    ///
    /// # Returns
    /// - `(spdx_id, license_name, confidence)` if confidence > 0.5
    /// - `None` if confidence is too low or no match found
    pub async fn detect_from_content(content: &str) -> Option<(String, String, f64)> {
        let store = get_license_store().await;
        let text_data = TextData::from(content);
        let result = store.analyze(&text_data);

        // Only accept matches with high confidence
        if result.score > 0.5 {
            Some((
                result.name.to_string(),
                result.name.to_string(),
                result.score as f64,
            ))
        } else {
            None
        }
    }
}

/// Convenience function for license detection
pub async fn detect_license(files: &[FileInfo]) -> Option<LicenseInfo> {
    LicenseDetector::find_license(files).await
}
