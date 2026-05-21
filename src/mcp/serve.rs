//! MCP server process management.
//! PID file-based tracking for start, stop, and status commands.

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::path::config_dir;

/// Information about a running MCP server instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeMcp {
    /// Process ID.
    pub pid: u32,
    /// Host address.
    pub host: String,
    /// Port number.
    pub port: u16,
    /// MCP protocol version.
    pub protocol: String,
    /// ISO 8601 timestamp the server was started.
    pub started_at: String,
}

impl ServeMcp {
    pub fn new(pid: u32, host: &str, port: u16, protocol: &str) -> Self {
        Self {
            pid,
            host: host.to_string(),
            port,
            protocol: protocol.to_string(),
            started_at: Local::now().to_rfc3339(),
        }
    }

    /// Read MCP server stats from the stats file.
    pub fn read() -> Result<Vec<ServeMcp>> {
        // Get stats file path
        let path = Self::stats_file();
        if !path.exists() {
            return Ok(Vec::new());
        }

        // Parse JSON content
        let content = fs::read_to_string(&path)?;
        let list: Vec<ServeMcp> = if content.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&content)?
        };

        // Filter alive processes and clean up dead entries
        let alive: Vec<ServeMcp> = list.clone().into_iter().filter(|s| s.is_alive()).collect();
        if alive.len() != list.len() {
            Self::write(&alive)?;
        }

        Ok(alive)
    }

    /// Write MCP server stats to the stats file.
    pub fn save(&self) -> Result<()> {
        let mut list = Self::read()?;
        // Replace existing entry with same PID, or push new
        if let Some(existing) = list.iter_mut().find(|s| s.pid == self.pid) {
            *existing = self.clone();
        } else {
            list.push(self.clone());
        }
        Self::write(&list)?;
        Ok(())
    }

    /// Calculate and format server uptime from started_at timestamp.
    pub fn load_time(&self) -> String {
        let started = chrono::DateTime::parse_from_rfc3339(&self.started_at)
            .unwrap_or_else(|_| chrono::DateTime::UNIX_EPOCH.into());
        let seconds = chrono::Utc::now()
            .signed_duration_since(started)
            .num_seconds()
            .max(0) as u64;
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            format!("{}m {}s", seconds / 60, seconds % 60)
        } else {
            format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
        }
    }

    /// Remove server entry by port from the stats file.
    fn remove(&self) -> Result<()> {
        let mut list = Self::read()?;
        list.retain(|s| s.pid != self.pid);
        Self::write(&list)?;
        Ok(())
    }

    /// Check if the process is still running.
    fn is_alive(&self) -> bool {
        std::process::Command::new("kill")
            .arg("-0")
            .arg(self.pid.to_string())
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Kill the server process with SIGKILL.
    fn kill(&self) -> Result<()> {
        std::process::Command::new("kill")
            .arg("-9")
            .arg(self.pid.to_string())
            .output()?;
        Ok(())
    }

    /// Kill the server process and remove it from the stats file.
    pub fn stop(&self) -> Result<()> {
        self.kill()?;
        self.remove()?;
        Ok(())
    }

    /// Write server list to the stats file.
    fn write(list: &[ServeMcp]) -> Result<()> {
        let path = Self::stats_file();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string(list)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Stats MCP file path: ~/.vibe-analyzer/servers.json
    fn stats_file() -> PathBuf {
        config_dir().join("serve.json")
    }
}
