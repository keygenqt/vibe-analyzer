//! SPDX license data utilities.
//! Downloads and maintains SPDX license list data from GitHub.

use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use zip::ZipArchive;

use crate::exit_error;
use crate::print_info;

const SPDX_REPO_URL: &str = "https://github.com/spdx/license-list-data/archive/refs/heads/main.zip";

/// Ensures SPDX data is available at the given path.
pub async fn ensure_data(data_path: &Path, is_default_path: bool) -> Result<()> {
    let details_path = data_path.join("json").join("details");

    if has_json_files(&details_path)? {
        return Ok(());
    }

    if !is_default_path {
        exit_error!(
            "SPDX data not found at: {}\n\
            Download from: {}",
            details_path.display(),
            SPDX_REPO_URL
        );
    }

    print_info!("SPDX data not found. Downloading...");
    download_spdx_data(data_path, &details_path).await
}

/// Checks if directory exists and contains JSON files.
fn has_json_files(dir: &Path) -> Result<bool> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(false);
    }
    let has_json = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .any(|e| e.path().extension().map_or(false, |ext| ext == "json"));
    Ok(has_json)
}

/// Downloads and extracts SPDX license data.
async fn download_spdx_data(data_path: &Path, details_path: &Path) -> Result<()> {
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
        copy_dir_all(&source_details, details_path)?;
    } else {
        anyhow::bail!("SPDX details directory not found");
    }

    Ok(())
}

/// Recursively copies a directory.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let dest = dst.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir_all(&entry.path(), &dest)?;
        } else {
            std::fs::copy(&entry.path(), &dest)?;
        }
    }
    Ok(())
}
