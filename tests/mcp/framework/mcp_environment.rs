//! Test environment management.
//! Provides setup and cleanup for OpenSearch test container and MCP server.

use anyhow::Context;
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;
use tracing::Level;

use crate::mcp::framework::mcp_environment_builder::McpEnvironmentBuilder;
use crate::mcp::framework::tee_writer::TeeWriter;

/// Test environment with OpenSearch container, MCP server, and fixtures.
#[derive(Debug)]
pub struct McpEnvironment {
    pub compose_file: String,
    pub opensearch_url: String,
    pub config_dir: String,
    pub vibe_analyzer_path: String,
    pub mcp_port: u16,
    pub mcp_server: Option<Child>,
    pub reports_dir: Option<PathBuf>,
    pub log_path: Option<PathBuf>,
    pub log_file: Option<Arc<Mutex<std::fs::File>>>,
    pub log_level: Level,
    pub request_timeout: Duration,
}

impl McpEnvironment {
    /// Creates a new builder for constructing the test environment.
    pub fn builder() -> McpEnvironmentBuilder {
        McpEnvironmentBuilder::new()
    }

    /// Returns MCP server URL.
    pub fn mcp_url(&self) -> String {
        format!("http://localhost:{}/mcp/v1", self.mcp_port)
    }

    /// Starts all services: OpenSearch container and MCP server.
    pub async fn start(&mut self) -> Result<()> {
        // Setup tracing with the configured log level
        let filter = format!(
            "mcp_test={0},vibe_analyzer={0}",
            self.log_level.as_str().to_lowercase()
        );
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stdout)
            .try_init();

        // Create reports directory with timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let reports_dir = PathBuf::from("tests/reports").join(&timestamp);
        std::fs::create_dir_all(&reports_dir)?;
        self.reports_dir = Some(reports_dir.clone());

        // Log file in reports directory
        let log_path = reports_dir.join("mcp_test.log");
        let log_file = std::fs::File::create(&log_path)?;
        let shared_file = Arc::new(Mutex::new(log_file));

        tracing::info!("Reports directory: {}", reports_dir.display());

        self.log_file = Some(shared_file.clone());
        self.log_path = Some(log_path);

        // Start OpenSearch
        tracing::info!("Start OpenSearch...");
        {
            let tee_writer = TeeWriter::new(shared_file.clone(), self.log_level);
            Command::new("docker")
                .args(["compose", "-f", &self.compose_file, "up", "-d"])
                .stdout(tee_writer.clone())
                .stderr(tee_writer)
                .status()?;
        }

        self.wait_for_opensearch().await?;

        // Start MCP server with logs redirected to both terminal and file
        tracing::info!("Start MCP server...");
        {
            let tee_writer = TeeWriter::new(shared_file.clone(), self.log_level);
            let child = Command::new(&self.vibe_analyzer_path)
                .args([
                    "--config-dir",
                    &self.config_dir,
                    "serve",
                    "start",
                    "--port",
                    &self.mcp_port.to_string(),
                ])
                .stdout(tee_writer.clone())
                .stderr(tee_writer)
                .spawn()?;
            self.mcp_server = Some(child);
        }

        // Wait for MCP server to become ready
        self.wait_for_mcp_server().await?;

        Ok(())
    }

    /// Waits for MCP server to become ready.
    async fn wait_for_mcp_server(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("http://localhost:{}/mcp/health", self.mcp_port);
        tracing::info!("Waiting for MCP server to become ready...");
        let start = std::time::Instant::now();
        let max_wait = self.request_timeout;
        loop {
            if start.elapsed() > max_wait {
                anyhow::bail!("MCP server failed to start within {:?}", max_wait);
            }
            match client
                .get(&url)
                .timeout(Duration::from_secs(1))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    tracing::info!("MCP server is ready after {:?}", start.elapsed());
                    return Ok(());
                }
                Ok(resp) => {
                    tracing::trace!("MCP server responded with status {}", resp.status());
                }
                Err(_) => {
                    // Still waiting
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }

    /// Indexes fixtures into OpenSearch using predefined config directory.
    pub async fn index_fixtures(&self) -> Result<()> {
        if let Some(shared) = &self.log_file {
            tracing::info!("Start indexing projects...");
            let tee_writer = TeeWriter::new(shared.clone(), self.log_level);
            Command::new(&self.vibe_analyzer_path)
                .args(["--config-dir", &self.config_dir])
                .args(["scan", "index"])
                .stdout(tee_writer.clone())
                .stderr(tee_writer)
                .status()?;

            // Wait for documents to become searchable
            self.wait_for_indexed_documents().await?;
        }
        Ok(())
    }

    /// Waits for indexed documents to become searchable in OpenSearch.
    async fn wait_for_indexed_documents(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/_refresh", self.opensearch_url);

        tracing::info!("Refreshing OpenSearch indices...");

        let response = client
            .post(&url)
            .timeout(self.request_timeout)
            .send()
            .await
            .with_context(|| "Failed to refresh OpenSearch indices")?;

        if response.status().is_success() {
            tracing::info!("OpenSearch indices refreshed successfully");
            Ok(())
        } else {
            anyhow::bail!(
                "OpenSearch refresh failed with status: {}",
                response.status()
            )
        }
    }

    /// Stops all services.
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.mcp_server.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        tracing::info!("Stop OpenSearch...");
        {
            Command::new("docker")
                .args(["compose", "-f", &self.compose_file, "down", "-v"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()?;
        }
        Ok(())
    }

    /// Waits for OpenSearch to become ready.
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

    /// Writes a structured log entry to the file.
    pub fn log(&self, message: &str, fields: &[(&str, &str)]) {
        if let Some(shared) = &self.log_file {
            let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
            let fields_str = fields
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(" ");
            let line = format!(
                "{} TRACE mcp_test::mcp::framework: {} {}\n",
                timestamp, message, fields_str
            );
            if let Ok(mut file) = shared.lock() {
                let _ = file.write_all(line.as_bytes());
                let _ = file.flush();
            }
        }
    }
}

/// Cleans up OpenSearch container and MCP server on drop.
impl Drop for McpEnvironment {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
