//! Builder for MCP test environment.
//! Provides step-by-step construction with validation.

use anyhow::Result;
use std::time::Duration;
use tracing::Level;

use crate::mcp::framework::mcp_environment::McpEnvironment;

/// Builder for constructing a validated MCP test environment.
#[derive(Debug)]
pub struct McpEnvironmentBuilder {
    compose_file: Option<String>,
    opensearch_url: Option<String>,
    config_dir: Option<String>,
    log_level: Option<Level>,
    request_timeout: Duration,
}

impl Default for McpEnvironmentBuilder {
    fn default() -> Self {
        Self {
            compose_file: None,
            opensearch_url: None,
            config_dir: None,
            log_level: None,
            request_timeout: Duration::from_secs(60),
        }
    }
}

impl McpEnvironmentBuilder {
    /// Creates a new builder with default empty values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the path to docker-compose file for OpenSearch.
    pub fn compose_file(mut self, compose_file: impl Into<String>) -> Self {
        self.compose_file = Some(compose_file.into());
        self
    }

    /// Sets the OpenSearch service URL.
    pub fn opensearch_url(mut self, opensearch_url: impl Into<String>) -> Self {
        self.opensearch_url = Some(opensearch_url.into());
        self
    }

    /// Sets the configuration directory path.
    pub fn config_dir(mut self, config_dir: impl Into<String>) -> Self {
        self.config_dir = Some(config_dir.into());
        self
    }

    /// Sets the log level for terminal output (default: INFO).
    #[allow(dead_code)]
    pub fn log_level(mut self, level: Level) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Sets the timeout for all external requests (default: 60 seconds).
    #[allow(dead_code)]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Validates and builds the MCP test environment.
    pub fn build(self) -> Result<McpEnvironment> {
        let compose_file = self
            .compose_file
            .ok_or_else(|| anyhow::anyhow!("compose_file not specified"))?;
        let opensearch_url = self
            .opensearch_url
            .ok_or_else(|| anyhow::anyhow!("opensearch_url not specified"))?;
        let config_dir = self
            .config_dir
            .ok_or_else(|| anyhow::anyhow!("config_dir not specified"))?;
        let log_level = self.log_level.unwrap_or(Level::INFO);

        let debug_path = "target/debug/vibe-analyzer";
        let vibe_analyzer_path = if std::path::Path::new(debug_path).exists() {
            debug_path.to_string()
        } else {
            "vibe-analyzer".to_string()
        };

        Ok(McpEnvironment {
            compose_file,
            opensearch_url,
            config_dir,
            vibe_analyzer_path,
            mcp_port: 9021,
            mcp_server: None,
            reports_dir: None,
            log_path: None,
            log_file: None,
            log_level,
            request_timeout: self.request_timeout,
        })
    }
}
