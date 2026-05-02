//! SPDX license detection using askalono.
//! Identifies open-source licenses from file content with confidence scoring.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use askalono::{Store, TextData};
use tempfile::tempdir;
use tokio::sync::OnceCell;
use zip::ZipArchive;

use crate::print_info;
use crate::utils::path::get_config_dir;

/// SPDX license list repository URL.
const SPDX_REPO_URL: &str = "https://github.com/spdx/license-list-data/archive/refs/heads/main.zip";

/// Global license store initialized on first access.
static STORE: OnceCell<Store> = OnceCell::const_new();

/// License detection result.
pub struct LicenseInfo {
    pub spdx_id: String,
    pub confidence: f64,
}

/// SPDX license detector.
pub struct LicenseDetector;

impl LicenseDetector {
    /// Detect license from content, downloading SPDX data on first call.
    pub async fn detect(content: &str) -> Option<LicenseInfo> {
        let store = STORE
            .get_or_init(|| async { Self::init_store().await })
            .await;
        let text_data = TextData::from(content);
        let result = store.analyze(&text_data);
        if result.score > 0.5 {
            Some(LicenseInfo {
                spdx_id: result.name.to_string(),
                confidence: result.score as f64,
            })
        } else {
            None
        }
    }

    /// Initialize license store from cache or download SPDX data.
    async fn init_store() -> Store {
        let cache_path = get_config_dir().join("askalono-cache.bin.zstd");
        if cache_path.exists() {
            let file = File::open(&cache_path).expect("Failed to open cache file");
            return Store::from_cache(file).expect("Failed to load cache");
        }

        let config = crate::configs::app::AppConfig::load().expect("Failed to load config");
        let details_path = config.dev.spdx_details_path();

        if !Self::has_json_files(&details_path) {
            print_info!("SPDX data not found. Downloading...");
            Self::download_spdx_data(&details_path)
                .await
                .expect("Failed to download SPDX data");
        }

        let mut store = Store::new();
        store
            .load_spdx(&details_path, false)
            .expect("Failed to load SPDX data");

        let cache_file = File::create(&cache_path).expect("Failed to create cache file");
        store.to_cache(cache_file).expect("Failed to save cache");

        store
    }

    fn has_json_files(dir: &Path) -> bool {
        dir.exists()
            && dir.is_dir()
            && std::fs::read_dir(dir).map_or(false, |entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            })
    }

    /// Download and extract SPDX license data from GitHub.
    async fn download_spdx_data(details_path: &Path) -> Result<()> {
        let data_path = details_path
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow::anyhow!("Invalid SPDX path"))?;

        std::fs::create_dir_all(data_path)?;

        let temp_dir = tempdir()?;
        let zip_path = temp_dir.path().join("spdx-data.zip");

        let response = reqwest::get(SPDX_REPO_URL).await?;
        let mut file = File::create(&zip_path)?;
        file.write_all(&response.bytes().await?)?;

        let zip_file = File::open(&zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        archive.extract(temp_dir.path())?;

        let extracted_dir = std::fs::read_dir(temp_dir.path())?
            .filter_map(|e| e.ok())
            .find(|e| {
                e.path().is_dir()
                    && e.file_name()
                        .to_string_lossy()
                        .contains("license-list-data")
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find extracted SPDX directory"))?;

        let source_details = extracted_dir.path().join("json").join("details");
        if source_details.exists() {
            Self::copy_dir_all(&source_details, details_path)?;
        } else {
            anyhow::bail!("SPDX details directory not found");
        }

        Ok(())
    }

    /// Recursively copy directory contents.
    fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let dest = dst.join(entry.file_name());
            if entry.path().is_dir() {
                Self::copy_dir_all(&entry.path(), &dest)?;
            } else {
                std::fs::copy(&entry.path(), &dest)?;
            }
        }
        Ok(())
    }
}
