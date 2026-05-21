//! Application configuration management for vibe-analyzer.
//! Handles loading, saving, and modifying knowledge sources with path normalization.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::configs::analyze::AnalyzeConfig;
use crate::configs::cluster::ClusterConfig;
use crate::configs::export::ExportConfig;
use crate::configs::log_level::LogLevel;
use crate::configs::mcp::McpConfig;
use crate::exit_error;
use crate::print_info;
use crate::utils;
use crate::utils::constants;
use crate::utils::json5::Json5;
use crate::utils::path::{self};

use super::opensearch::OpenSearchConfig;

/// Global config path override.
static GLOBAL_CONFIG_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version.
    pub version: String,
    /// Log level for terminal/tracing output.
    pub log_level: LogLevel,
    /// OpenSearch connection settings.
    pub opensearch: OpenSearchConfig,
    /// MCP server settings.
    pub mcp: McpConfig,
    /// LLM enrichment settings.
    pub analyze: AnalyzeConfig,
    /// Export settings.
    pub export: ExportConfig,
    /// LLM cluster nodes (local and cloud models).
    pub cluster: Vec<ClusterConfig>,
    /// Knowledge sources to index.
    pub sources: Vec<PathBuf>,
}

impl Default for AppConfig {
    /// Creates default configuration with safe defaults.
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            log_level: LogLevel::default(),
            opensearch: OpenSearchConfig::default(),
            mcp: McpConfig::default(),
            analyze: AnalyzeConfig::default(),
            export: ExportConfig::default(),
            cluster: vec![ClusterConfig::default()],
            sources: vec![],
        }
    }
}

impl AppConfig {
    /// Sets the global config path override. Must be a valid JSON5 file.
    pub fn set_global_config(path: Option<PathBuf>) {
        if let Some(ref p) = path {
            if !p.exists() {
                exit_error!("Config file not found: {}", p.display());
            }
            if p.extension().map_or(true, |ext| ext != "json5") {
                exit_error!("Config file must be .json5: {}", p.display());
            }
        } else {
            if let Err(err) = Self::load() {
                exit_error!("{}", err);
            }
        }
        if GLOBAL_CONFIG_PATH.get().is_none() {
            let _ = GLOBAL_CONFIG_PATH.set(path);
        }
    }

    /// Returns the config file path.
    pub fn config_path() -> PathBuf {
        Self::config_dir().join(constants::CONFIG_FILE_NAME)
    }

    /// Returns the config directory (either global override or default).
    pub fn config_dir() -> PathBuf {
        if let Some(Some(config_path)) = GLOBAL_CONFIG_PATH.get() {
            config_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| path::config_dir())
        } else {
            path::config_dir()
        }
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
                Err(e) => {
                    println!("{:?}", e);

                    return Err(anyhow::anyhow!(
                        "Failed to parse config file (possibly from older version).\nDelete {} and run again to create a new one.",
                        config_path.display()
                    ));
                }
            };

            if config.version != constants::CONFIG_VERSION {
                return Err(anyhow::anyhow!(
                    "Config version mismatch: found {}, expected {}\nDelete {} and run again to create a new one.",
                    config.version,
                    constants::CONFIG_VERSION,
                    config_path.display()
                ));
            }

            // Normalize paths on load
            for source in &mut config.sources {
                if let Ok(abs_path) = vibe_fs::resolve(&source) {
                    *source = abs_path;
                }
            }

            config
        };

        // Initialize tracing with log level from config
        Self::init_tracing(&config.log_level);

        Ok(config)
    }

    /// Initializes tracing subscriber with the given log level.
    fn init_tracing(level: &LogLevel) {
        crate::utils::macros::set_log_level(level);
        if crate::utils::macros::is_tracing() {
            let filter = format!("vibe_analyzer={0},mcp_test={0}", level.as_str());
            let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        }
    }

    /// Save config to file in JSON5 format with helpful comments.
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        let commits = vec![
            Json5::Commit("version", vec!["Configuration version (do not modify)"]),
            Json5::Commit(
                "log_level",
                vec!["Log level: cli, error, warn, info, debug, trace"],
            ),
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
                "analyze",
                vec![
                    "Analysis settings for scan commands.",
                    "When all features are disabled, only AST data is returned.",
                    "",
                    "max_chunk_chars - Maximum characters per request batch",
                    "",
                    "Enrichment features:",
                    "   meta   - Generate summary and tags for the file",
                    "   debt   - Detect technical debt: TODO, magic numbers, commented-out code",
                    "   advice - Suggest refactoring, naming, and test improvements",
                    "   errors - Find bugs, spelling errors, and potential issues",
                ],
            ),
            Json5::Commit(
                "export",
                vec![
                    "Default export settings for scan ast and scan analyze.",
                    "",
                    "format - Default output format: json, json5, toml, toon, xml, yaml",
                    "output_dir - Default output directory",
                    "",
                    "File types to include:",
                    "   code - Source code files (Rust, Python, JS, etc.)",
                    "   markdown - Markdown documentation files",
                    "   text - Text files without AST parser (txt, logs, etc.)",
                    "   binary - Binary files (images, etc.)",
                    "",
                    "AST elements to include:",
                    "   imports - Import statements",
                    "   functions - Function signatures",
                    "   variables - Variable declarations",
                    "   enums - Enum declarations",
                    "   interfaces - Interface, trait declarations",
                    "   classes - Class declarations",
                    "   structs - Struct declarations",
                    "   comments - Comments on elements",
                    "   header_comments - File header comments",
                    "   headings - Markdown headings",
                    "   links - Markdown links",
                    "   code_blocks - Markdown code blocks",
                    "   markers - Markers from static analysis (TODO, FIXME, HACK, etc.)",
                    "   warnings - Warnings from static analysis (unwrap, panic, bare except, etc.)",
                    "   body - Body content (functions, classes, headings, etc.)",
                ],
            ),
            Json5::Commit(
                "cluster",
                vec![
                    "LLM cluster nodes (local and cloud models)",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen",
                    "host - API endpoint (default: http://localhost:11434 for Ollama)",
                    "model - LLM model name",
                    "timeout_secs - Request timeout in seconds",
                    "temperature - Sampling temperature (0.0 - 1.0)",
                    "seed - Random seed for reproducibility",
                    "num_ctx - Context window size",
                    "num_predict - Maximum tokens to generate",
                    "api_key - API key for cloud providers (not needed for Ollama)",
                    "parallel - Number of parallel connections to this provider (default 1)",
                ],
            ),
            Json5::Commit("sources", vec!["Sources to index"]),
        ];
        let json = json5::to_string(self)?;
        let content = utils::json5::add_commits(json, commits);
        fs::write(Self::config_path(), content)?;
        Ok(())
    }

    /// Add source to config.
    pub fn add_source(&mut self, path: PathBuf) {
        let absolute_path = vibe_fs::resolve(&path).unwrap_or(path);
        if !self.sources.contains(&absolute_path) {
            self.sources.push(absolute_path);
            let _ = self.save();
        }
    }

    /// Remove source from config by target path.
    pub fn remove_source(&mut self, target: &str) -> Result<PathBuf> {
        let search_path = vibe_fs::resolve(target).unwrap_or_else(|_| PathBuf::from(target));
        if let Some(pos) = self.sources.iter().position(|p| {
            *p == search_path || p.ends_with(target) || p.to_string_lossy().ends_with(target)
        }) {
            let removed = self.sources.remove(pos);
            self.save()?;
            Ok(removed)
        } else {
            Err(anyhow::anyhow!("Source not found: {}", target))
        }
    }

    /// Get source by exact path or unique directory name.
    pub fn get_source(&self, target: &str) -> Result<PathBuf> {
        let path = PathBuf::from(target);
        if path.is_absolute() && path.exists() && path.is_dir() {
            return Ok(path);
        }
        let sources: Vec<PathBuf> = self
            .sources
            .iter()
            .filter_map(|s| vibe_fs::resolve(s).ok())
            .filter(|s| s.exists())
            .collect();
        let found: Vec<&PathBuf> = sources
            .iter()
            .filter(|source| {
                source
                    .to_string_lossy()
                    .split('/')
                    .last()
                    .map(|name| name == target)
                    .unwrap_or(false)
            })
            .collect();
        if found.is_empty() {
            return Err(anyhow::anyhow!("Source not found: {}", target));
        }
        if found.len() > 1 {
            return Err(anyhow::anyhow!(
                "Multiple sources found with name '{}'",
                target
            ));
        }
        Ok(found[0].clone())
    }

    /// Get list of valid sources.
    pub fn get_sources(&self) -> Vec<PathBuf> {
        self.sources
            .iter()
            .filter_map(|p| vibe_fs::resolve(p).ok())
            .filter(|p| p.exists())
            .collect()
    }
}
