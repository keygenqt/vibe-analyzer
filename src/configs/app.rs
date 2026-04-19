//! Application configuration management for vibe-analyzer.
//! Handles loading, saving, and modifying knowledge sources with path normalization.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::configs::dev::DevConfig;
use crate::configs::dev::LogLevel;
use crate::configs::mcp::McpConfig;
use crate::print_info;
use crate::utils;
use crate::utils::app;
use crate::utils::constants;
use crate::utils::hash::hash_from_path;
use crate::utils::json5::Json5;
use crate::utils::path::convert_relative;
use crate::utils::path::{self};

use super::ollama::OllamaConfig;
use super::opensearch::OpenSearchConfig;

/// Global config directory override.
static GLOBAL_CONFIG_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

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
    pub mcp: McpConfig,
    pub ollama: Vec<OllamaConfig>,
    pub sources: Vec<PathBuf>,

    /// Development and debugging configuration
    #[serde(default, skip_serializing_if = "DevConfig::should_skip")]
    pub dev: DevConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            opensearch: OpenSearchConfig::default(),
            mcp: McpConfig::default(),
            ollama: vec![OllamaConfig::default()],
            sources: vec![],
            dev: DevConfig::default(),
        }
    }
}

impl AppConfig {
    /// Sets the global config directory override.
    pub fn set_global_config_dir(dir: Option<PathBuf>) {
        if GLOBAL_CONFIG_DIR.get().is_none() {
            let _ = GLOBAL_CONFIG_DIR.set(dir);
        }
    }

    /// Returns the config directory (either global override or default).
    pub fn config_dir() -> PathBuf {
        if let Some(Some(dir)) = GLOBAL_CONFIG_DIR.get() {
            dir.clone()
        } else {
            path::get_config_dir()
        }
    }

    /// Returns the config file path.
    pub fn config_path() -> PathBuf {
        Self::config_dir().join(constants::CONFIG_FILE_NAME)
    }

    /// Load config from file, respecting global override.
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        let config = if !config_path.exists() {
            print_info!("Config not found, creating default");
            let config = AppConfig::default();
            config.save()?;
            config
        } else {
            let content = fs::read_to_string(&config_path)?;
            let mut config: AppConfig = match json5::from_str(&content) {
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

            config
        };

        // Initialize tracing with log level from config
        Self::init_tracing(&config.dev.log_level);

        Ok(config)
    }

    /// Initializes tracing subscriber with the given log level.
    fn init_tracing(level: &LogLevel) {
        crate::utils::macros::set_log_level(level);
        if crate::utils::macros::is_tracing() {
            let filter = format!("vibe_analyzer={0},mcp_test={0}", level.as_str());
            tracing_subscriber::fmt().with_env_filter(filter).init();
        }
    }

    /// Save config to file in JSON5 format with helpful comments.
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let mut commits = vec![
            Json5::Commit("version", vec!["Configuration version (do not modify)"]),
            Json5::Commit("opensearch", vec!["OpenSearch connection"]),
            Json5::Commit(
                "mcp",
                vec![
                    "MCP server configuration",
                    "",
                    "host - Bind address (0.0.0.0 for all interfaces, 127.0.0.1 for local only)",
                    "port - Server port (default: 9020)",
                    "protocol - MCP protocol version (2024-11-05, 2025-03-26, 2025-06-18, or 'latest')",
                ],
            ),
            Json5::Commit(
                "ollama",
                vec![
                    "Ollama LLM servers",
                    "",
                    "host - API endpoint (default: http://localhost:11434)",
                    "model - LLM model name",
                    "max_chunk_chars - Maximum characters per request batch",
                    "max_chunk_files - Maximum files per request batch",
                    "timeout_secs - Request timeout in seconds",
                    "temperature - Sampling temperature (0.0 - 1.0)",
                    "seed - Random seed for reproducibility",
                    "num_ctx - Context window size",
                    "num_predict - Maximum tokens to generate",
                    "ast_imports - Include imports in AST analysis",
                    "ast_variables - Include variables in AST analysis",
                    "ast_functions - Include functions in AST analysis",
                    "ast_enums - Include enums in AST analysis",
                    "ast_interfaces - Include interfaces in AST analysis",
                ],
            ),
            Json5::Commit("sources", vec!["Knowledge sources to index"]),
        ];

        // Add dev block with comments only if it existed in the original config
        if self.dev.exists_in_config() {
            commits.push(Json5::Commit(
                "dev",
                vec![
                    "Development configuration",
                    "",
                    "log_level - Log level: default, trace, debug, info, warn, error",
                    "spdx_data_path - Override SPDX data path for license detection",
                ],
            ));
        }

        let json = json5::to_string(self)?;
        let content = utils::json5::add_commits(json, commits);
        fs::write(Self::config_path(), content)?;
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
