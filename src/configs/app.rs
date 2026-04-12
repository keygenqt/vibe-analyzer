//! Application configuration management for vibe-analyzer.
//! Handles loading, saving, and modifying knowledge sources with path normalization.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::configs::spdx::SpdxConfig;
use crate::print_info;
use crate::utils::app;
use crate::utils::constants;
use crate::utils::hash::hash_from_path;
use crate::utils::path::convert_relative;
use crate::utils::path::{self};

use super::ollama::OllamaConfig;
use super::opensearch::OpenSearchConfig;

/// Type of OpenSearch index.
#[allow(dead_code)]
pub enum IndexType {
    /// Metadata index (shared across all sources).
    Meta,
    /// Files content index.
    Files,
    /// Files analysis index (AST + enrichment).
    Analysis,
    /// All index types.
    All,
}

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub opensearch: OpenSearchConfig,
    pub ollama: Vec<OllamaConfig>,
    pub spdx: SpdxConfig,
    pub sources: Vec<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            opensearch: OpenSearchConfig::default(),
            ollama: vec![OllamaConfig::default()],
            spdx: SpdxConfig::default(),
            sources: vec![],
        }
    }
}

impl AppConfig {
    /// Load config from file, create default if not exists.
    pub fn load() -> Result<Self> {
        let config_path = path::get_config_path();

        if !config_path.exists() {
            print_info!("Config not found, creating default");
            let config = AppConfig::default();
            config.save()?;
            Ok(config)
        } else {
            let content = fs::read_to_string(&config_path)?;
            let mut config: AppConfig = match serde_json::from_str(&content) {
                Ok(config) => config,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse config file (possibly from older version).\n\
                     Delete {} and run again to create a new one.",
                        config_path.display()
                    ));
                }
            };

            if config.version != constants::CONFIG_VERSION {
                return Err(anyhow::anyhow!(
                    "Config version mismatch: found {}, expected {}\n\
                 Delete {} and run again to create a new one.",
                    config.version,
                    constants::CONFIG_VERSION,
                    config_path.display()
                ));
            }

            // Normalize paths on load
            for source in &mut config.sources {
                let path_str = source.to_string_lossy();
                if let Some(abs_path) = convert_relative(&path_str) {
                    *source = abs_path;
                }
            }

            Ok(config)
        }
    }

    /// Save config to file.
    pub fn save(&self) -> Result<()> {
        let config_dir = path::get_config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path::get_config_path(), content)?;
        Ok(())
    }

    /// Add source to config.
    pub fn add_source(&mut self, path: PathBuf) {
        let path_str = path.to_string_lossy();
        let absolute_path = match convert_relative(&path_str) {
            Some(p) => p,
            None => path,
        };

        if !self.sources.contains(&absolute_path) {
            self.sources.push(absolute_path);
            let _ = self.save();
        }
    }

    /// Remove source from config by target path.
    pub fn remove_source(&mut self, target: &str) -> Result<PathBuf> {
        let normalized_target = match convert_relative(target) {
            Some(p) => p,
            None => PathBuf::from(target),
        };

        if let Some(pos) = self.sources.iter().position(|p| *p == normalized_target) {
            let removed = self.sources.remove(pos);
            self.save()?;
            Ok(removed)
        } else {
            Err(anyhow::anyhow!(
                "Source not found: {}",
                normalized_target.display()
            ))
        }
    }

    /// Get source by exact path or unique directory name.
    pub fn get_source(&self, target: &str) -> Result<PathBuf> {
        let path = PathBuf::from(target);
        let path: Vec<&PathBuf> = if path.exists() && path.is_dir() {
            vec![&path]
        } else {
            self.sources
                .iter()
                .filter(|source| {
                    source
                        .to_string_lossy()
                        .split('/')
                        .last()
                        .map(|name| name == target)
                        .unwrap_or(false)
                })
                .collect()
        };

        if path.is_empty() {
            return Err(anyhow::anyhow!("Source not found: {}", target));
        }

        if path.len() > 1 {
            return Err(anyhow::anyhow!(
                "Multiple sources found with name '{}'. Use full path instead.",
                target
            ));
        }

        Ok(path.first().unwrap().to_path_buf())
    }

    /// Get list of valid sources.
    pub fn get_sources(&self) -> Vec<PathBuf> {
        self.sources
            .iter()
            .filter(|p| p.exists())
            .cloned()
            .collect()
    }

    /// Get meta index name.
    pub fn meta_index() -> String {
        format!("{}-meta", app::app_name())
    }

    /// Get files index name for a source.
    pub fn files_index(root: &PathBuf) -> String {
        format!("{}-files-{}", app::app_name(), hash_from_path(root))
    }

    /// Get files analysis index name for a source.
    pub fn files_analysis_index(root: &PathBuf) -> String {
        format!(
            "{}-files-analysis-{}",
            app::app_name(),
            hash_from_path(root)
        )
    }

    /// Get index name for a specific source by target.
    /// Get index name for a specific source by target.
    pub fn get_index(&self, target: &str, index_type: IndexType) -> Result<String> {
        let source = self.get_source(target)?;
        Ok(match index_type {
            IndexType::Meta => Self::meta_index(),
            IndexType::Files => Self::files_index(&source),
            IndexType::Analysis => Self::files_analysis_index(&source),
            IndexType::All => unreachable!("All not supported for single source"),
        })
    }

    /// Get indexes names for all sources.
    pub fn get_indexes(&self, index_type: IndexType) -> Vec<String> {
        match index_type {
            IndexType::Meta => vec![Self::meta_index()],
            IndexType::Files => self
                .get_sources()
                .iter()
                .map(|source| Self::files_index(source))
                .collect(),
            IndexType::Analysis => self
                .get_sources()
                .iter()
                .map(|source| Self::files_analysis_index(source))
                .collect(),
            IndexType::All => {
                let mut indexes = vec![Self::meta_index()];
                for source in self.get_sources() {
                    indexes.push(Self::files_index(&source));
                    indexes.push(Self::files_analysis_index(&source));
                }
                indexes
            }
        }
    }
}
