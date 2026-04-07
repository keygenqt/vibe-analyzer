//! SPDX license data management for license detection.
//! Downloads and maintains SPDX license list data from GitHub for accurate license identification.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::tempdir;
use zip::ZipArchive;

use crate::exit_error;
use crate::print_info;
use crate::utils::path::get_config_dir;

/// URL to the official SPDX license list data repository
const SPDX_REPO_URL: &str = "https://github.com/spdx/license-list-data/archive/refs/heads/main.zip";

/// Configuration for SPDX license data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpdxConfig {
    /// Path to the directory containing SPDX license data
    pub data_path: String,
}

impl Default for SpdxConfig {
    /// Creates default SPDX configuration
    fn default() -> Self {
        Self {
            data_path: get_config_dir().join("spdx").to_string_lossy().to_string(),
        }
    }
}

impl SpdxConfig {
    /// Returns the path to the directory containing license JSON files
    pub fn get_spdx_path(&self) -> PathBuf {
        Path::new(&self.data_path).join("json").join("details")
    }

    /// Checks if the configured path is the default one
    fn is_default_path(&self) -> bool {
        self.data_path == SpdxConfig::default().data_path
    }

    /// Ensures SPDX data is available, downloading if necessary
    pub async fn ensure_data(&self) -> Result<()> {
        let spdx_path = self.get_spdx_path();

        // Check if data already exists
        if spdx_path.exists() && spdx_path.is_dir() {
            let has_json = std::fs::read_dir(&spdx_path)?
                .filter_map(|entry| entry.ok())
                .any(|entry| entry.path().extension().map_or(false, |ext| ext == "json"));

            if has_json {
                return Ok(());
            }
        }

        // For custom paths, fail with clear instructions
        if !self.is_default_path() {
            exit_error!(
                "SPDX data not found at: {}\n\
                Please download SPDX license data to this path or reset spdx.data_path in config.\n\
                Download from: {}",
                spdx_path.display(),
                SPDX_REPO_URL
            );
        }

        // Auto-download for default path
        print_info!("SPDX data not found. Downloading...");
        self.download_spdx_data().await?;

        Ok(())
    }

    /// Downloads and extracts SPDX license data from GitHub
    async fn download_spdx_data(&self) -> Result<()> {
        let target_dir = Path::new(&self.data_path);
        let details_path = self.get_spdx_path();

        std::fs::create_dir_all(target_dir)?;

        let temp_dir = tempdir()?;
        let zip_path = temp_dir.path().join("spdx-data.zip");

        let response = reqwest::get(SPDX_REPO_URL).await?;
        let bytes = response.bytes().await?;

        let mut file = File::create(&zip_path)?;
        file.write_all(&bytes)?;

        let zip_file = File::open(&zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        archive.extract(temp_dir.path())?;

        let extracted_dir = std::fs::read_dir(temp_dir.path())?
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                entry.path().is_dir()
                    && entry
                        .file_name()
                        .to_string_lossy()
                        .contains("license-list-data")
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find extracted SPDX directory"))?;

        let source_details = extracted_dir.path().join("json").join("details");
        if source_details.exists() {
            self.copy_dir_all(&source_details, &details_path)?;
        } else {
            anyhow::bail!("SPDX details directory not found in downloaded archive");
        }

        Ok(())
    }

    /// Recursively copies a directory and all its contents
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dst.join(entry.file_name());

            if path.is_dir() {
                self.copy_dir_all(&path, &dest_path)?;
            } else {
                std::fs::copy(&path, &dest_path)?;
            }
        }
        Ok(())
    }
}
