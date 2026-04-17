//! Test environment management.
//! Provides setup and cleanup for OpenSearch test container and MCP server.

use anyhow::Result;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

/// Test environment with OpenSearch container, MCP server, and fixtures.
#[derive(Debug)]
pub struct McpEnvironment {
    compose_file: String,
    opensearch_url: String,
    config_dir: String,
    vibe_analyzer_path: String,
    mcp_port: u16,
    mcp_server: Option<Child>,
}

impl McpEnvironment {
    /// Creates a new test environment.
    pub fn new(
        compose_file: impl Into<String>,
        opensearch_url: impl Into<String>,
        config_dir: impl Into<String>,
    ) -> Self {
        let release_path = "target/release/vibe-analyzer";
        let debug_path = "target/debug/vibe-analyzer";
        let vibe_analyzer_path = if std::path::Path::new(release_path).exists() {
            release_path.to_string()
        } else if std::path::Path::new(debug_path).exists() {
            debug_path.to_string()
        } else {
            "vibe-analyzer".to_string()
        };
        Self {
            compose_file: compose_file.into(),
            opensearch_url: opensearch_url.into(),
            config_dir: config_dir.into(),
            vibe_analyzer_path,
            mcp_port: 9021,
            mcp_server: None,
        }
    }

    /// Returns MCP server URL.
    pub fn mcp_url(&self) -> String {
        format!("http://localhost:{}/mcp/v1", self.mcp_port)
    }

    /// Starts all services: OpenSearch container and MCP server.
    pub async fn start(&mut self) -> Result<()> {
        // Start OpenSearch
        Command::new("docker")
            .args(["compose", "-f", &self.compose_file, "up", "-d"])
            .status()?;

        self.wait_for_opensearch().await?;

        // Start MCP server
        let child = Command::new(&self.vibe_analyzer_path)
            .args([
                "--config-dir",
                &self.config_dir,
                "serve",
                "start",
                "--port",
                &self.mcp_port.to_string(),
            ])
            .spawn()?;

        self.mcp_server = Some(child);
        std::thread::sleep(Duration::from_secs(2));

        Ok(())
    }

    /// Indexes fixtures into OpenSearch using predefined config directory.
    pub fn index_fixtures(&self) -> Result<()> {
        Command::new(&self.vibe_analyzer_path)
            .args(["--config-dir", &self.config_dir])
            .args(["scan", "index"])
            .status()?;
        Ok(())
    }

    /// Stops all services.
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.mcp_server.take() {
            let _ = child.kill();
            let _ = child.wait();
        }

        Command::new("docker")
            .args(["compose", "-f", &self.compose_file, "down", "-v"])
            .status()?;

        Ok(())
    }

    async fn wait_for_opensearch(&self) -> Result<()> {
        let client = reqwest::Client::new();
        for _ in 0..30 {
            if client.get(&self.opensearch_url).send().await.is_ok() {
                return Ok(());
            }
            sleep(Duration::from_secs(2)).await;
        }
        anyhow::bail!("OpenSearch failed to start")
    }
}

impl Drop for McpEnvironment {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
