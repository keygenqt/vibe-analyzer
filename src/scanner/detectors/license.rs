//! License detection using SPDX data and askalono
//!
//! This module provides lazy initialization of the license store,
//! automatic SPDX data download on first use, and caching for performance.

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
/// # Returns
/// * `&'static Store` - Reference to the license store
async fn get_license_store() -> &'static Store {
    LICENSE_STORE.get_or_init(init_license_store).await
}

/// Initializes the license store from cache or downloads SPDX data
///
/// # Returns
/// * `Store` - Initialized license store
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

        store
            .load_spdx(&details_path, false)
            .expect("Failed to load SPDX data");

        let cache_file = File::create(cache_path).expect("Failed to create cache file");
        store.to_cache(cache_file).expect("Failed to save cache");

        store
    }
}

/// License detector
pub struct LicenseDetector;

impl LicenseDetector {
    /// Searches for license file and detects license type
    ///
    /// # Arguments
    /// * `files` - List of files to search for license
    ///
    /// # Returns
    /// * `Option<LicenseInfo>` - License information if found and detected
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

    /// Detects license from content, returns (spdx_id, name, confidence) if confidence > 0.5
    ///
    /// # Arguments
    /// * `content` - License text content
    ///
    /// # Returns
    /// * `Option<(String, String, f64)>` - SPDX ID, license name, and confidence score
    pub async fn detect_from_content(content: &str) -> Option<(String, String, f64)> {
        let store = get_license_store().await;
        let text_data = TextData::from(content);
        let result = store.analyze(&text_data);

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
///
/// # Arguments
/// * `files` - List of files to search for license
///
/// # Returns
/// * `Option<LicenseInfo>` - License information if found and detected
pub async fn detect_license(files: &[FileInfo]) -> Option<LicenseInfo> {
    LicenseDetector::find_license(files).await
}
