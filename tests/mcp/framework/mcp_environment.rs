//! Test environment management.
//! Provides setup and cleanup for OpenSearch test container and MCP server.

use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;
use vibe_analyzer::configs::app::AppConfig;

/// Test environment with OpenSearch container, MCP server, and fixtures.
#[derive(Debug)]
pub struct McpEnvironment {
    compose_file: String,
    opensearch_url: String,
    config_dir: String,
    vibe_analyzer_path: String,
    mcp_port: u16,
    mcp_server: Option<Child>,
    log_dir: Option<TempDir>,
    pub log_path: Option<PathBuf>,
    log_file: Option<Arc<Mutex<std::fs::File>>>,
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
            log_dir: None,
            log_path: None,
            log_file: None,
        }
    }

    /// Returns MCP server URL.
    pub fn mcp_url(&self) -> String {
        format!("http://localhost:{}/mcp/v1", self.mcp_port)
    }

    /// Starts all services: OpenSearch container and MCP server.
    pub async fn start(&mut self) -> Result<()> {
        // Initialize tracing for the test runner
        AppConfig::set_global_config_dir(Some(PathBuf::from(&self.config_dir)));
        let _ = AppConfig::load()?;

        // Create temp dir for logs
        let log_dir = TempDir::new()?;
        let log_path = log_dir.path().join("mcp_test.log");
        let log_file = std::fs::File::create(&log_path)?;
        let shared_file = Arc::new(Mutex::new(log_file));

        self.log_file = Some(shared_file.clone());
        self.log_dir = Some(log_dir);
        self.log_path = Some(log_path);

        // Start OpenSearch
        tracing::info!("Start OpenSearch...");
        {
            let cmd_file = shared_file.lock().unwrap().try_clone()?;
            Command::new("docker")
                .args(["compose", "-f", &self.compose_file, "up", "-d"])
                .stdout(cmd_file.try_clone()?)
                .stderr(cmd_file)
                .status()?;
        }

        self.wait_for_opensearch().await?;

        // Start MCP server with logs redirected
        tracing::info!("Start MCP server...");
        {
            let cmd_file = shared_file.lock().unwrap().try_clone()?;
            let child = Command::new(&self.vibe_analyzer_path)
                .args([
                    "--config-dir",
                    &self.config_dir,
                    "serve",
                    "start",
                    "--port",
                    &self.mcp_port.to_string(),
                ])
                .stdout(cmd_file.try_clone()?)
                .stderr(cmd_file)
                .spawn()?;
            self.mcp_server = Some(child);
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
        Ok(())
    }

    /// Indexes fixtures into OpenSearch using predefined config directory.
    pub fn index_fixtures(&self) -> Result<()> {
        if let Some(shared) = &self.log_file {
            tracing::info!("Start indexing projects...");
            let cmd_file = shared.lock().unwrap().try_clone()?;
            Command::new(&self.vibe_analyzer_path)
                .args(["--config-dir", &self.config_dir])
                .args(["scan", "index"])
                .stdout(cmd_file.try_clone()?)
                .stderr(cmd_file)
                .status()?;
        }
        Ok(())
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
