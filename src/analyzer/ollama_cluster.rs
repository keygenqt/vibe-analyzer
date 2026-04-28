//! Ollama cluster for load balancing across multiple Ollama servers.
//! Uses competing consumers via shared mpsc channel for parallel file processing.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::analyzer::ollama_client::OllamaClient;
use crate::configs::ollama::OllamaConfig;
use anyhow::Result;

pub struct OllamaCluster {
    /// List of available Ollama clients
    pub clients: Vec<OllamaClient>,
    /// Round-robin counter for next client selection
    current: AtomicUsize,
}

impl OllamaCluster {
    /// Creates a new cluster from a list of Ollama configurations.
    pub fn new(configs: &Vec<OllamaConfig>) -> Self {
        let clients = configs
            .into_iter()
            .map(|config| OllamaClient::new(&config))
            .collect();

        Self {
            clients,
            current: AtomicUsize::new(0),
        }
    }

    /// Returns the next client order.
    pub fn next(&self) -> &OllamaClient {
        let idx = self.current.fetch_add(1, Ordering::Relaxed) % self.clients.len();
        &self.clients[idx]
    }

    /// Returns true if cluster has no clients.
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    /// Returns the number of clients in the cluster.
    pub fn len(&self) -> usize {
        self.clients.len()
    }

    /// Checks if all Ollama servers are available.
    pub async fn is_available(&self, on_progress: fn(usize, usize)) -> Result<bool> {
        let total = self.clients.len();

        for (i, client) in self.clients.iter().enumerate() {
            on_progress(i + 1, total);

            if !client.is_available().await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Ensures all Ollama servers are ready with progress reporting.
    pub async fn ensure_ready(&self, on_progress: fn(usize, usize)) -> Result<()> {
        let total = self.clients.len();

        for (i, client) in self.clients.iter().enumerate() {
            on_progress(i + 1, total);

            if let Err(e) = client.ensure_ready().await {
                anyhow::bail!("Server {} ({}): {}", i + 1, client.config.host, e);
            }
        }

        Ok(())
    }
}
