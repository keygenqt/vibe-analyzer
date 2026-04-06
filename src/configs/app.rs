//! Application configuration management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::configs::spdx::SpdxConfig;
use crate::print_info;
use crate::utils::path::{self, convert_relative};

use super::ollama::OllamaConfig;
use super::opensearch::OpenSearchConfig;

/// Main application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub opensearch: OpenSearchConfig,
    pub ollama: OllamaConfig,
    pub spdx: SpdxConfig,
    pub projects: Vec<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            opensearch: OpenSearchConfig::default(),
            ollama: OllamaConfig::default(),
            spdx: SpdxConfig::default(),
            projects: vec![],
        }
    }
}

impl AppConfig {
    /// Load config from file, create default if not exists
    pub fn load() -> Result<Self> {
        let config_path = path::get_config_path();

        if !config_path.exists() {
            print_info!("Config not found, creating default");
            let config = AppConfig::default();
            config.save()?;
            Ok(config)
        } else {
            let content = fs::read_to_string(&config_path)?;
            let mut config: AppConfig = serde_json::from_str(&content)?;

            // Normalize paths on load using convert_relative
            for project in &mut config.projects {
                let path_str = project.to_string_lossy();
                if let Some(abs_path) = convert_relative(&path_str) {
                    *project = abs_path;
                }
            }

            Ok(config)
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_dir = path::get_config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path::get_config_path(), content)?;
        Ok(())
    }

    /// Add project to config
    pub fn add_project(&mut self, path: PathBuf) {
        let path_str = path.to_string_lossy();
        let absolute_path = match convert_relative(&path_str) {
            Some(p) => p,
            None => path,
        };

        if !self.projects.contains(&absolute_path) {
            self.projects.push(absolute_path);
            let _ = self.save();
        }
    }

    /// Delete project from config
    pub fn del_project(&mut self, path: PathBuf) -> Result<PathBuf> {
        let path_str = path.to_string_lossy();
        let normalized_path = match convert_relative(&path_str) {
            Some(p) => p,
            None => path,
        };

        if let Some(pos) = self.projects.iter().position(|p| *p == normalized_path) {
            let removed = self.projects.remove(pos);
            self.save()?;
            Ok(removed)
        } else {
            Err(anyhow::anyhow!(
                "Project not found: {}",
                normalized_path.display()
            ))
        }
    }

    /// Get list of valid projects
    pub fn get_projects(&self) -> Vec<PathBuf> {
        self.projects
            .iter()
            .filter(|p| p.exists())
            .cloned()
            .collect()
    }
}
