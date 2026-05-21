//! LLM provider configuration for model inference.
//! Defines connection parameters and model settings for code analysis.

use serde::{Deserialize, Serialize};

/// Configuration for a single LLM provider connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Provider type: "ollama", "deepseek", "qwen".
    pub provider: String,
    /// API endpoint URL.
    pub host: String,
    /// Model name to use for inference.
    pub model: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Temperature for generation (0.0-2.0, lower = more deterministic).
    pub temperature: f32,
    /// Seed for reproducible outputs.
    pub seed: u64,
    /// Context window size in tokens.
    pub num_ctx: usize,
    /// Maximum tokens to predict/generate.
    pub num_predict: usize,
    /// API key for cloud providers (not needed for Ollama).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Number of parallel connections to this provider (default 1).
    pub parallel: usize,
}

impl Default for ClusterConfig {
    /// Creates default Ollama configuration.
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:3b-instruct".to_string(),
            timeout_secs: 60,
            temperature: 0.1,
            seed: 42,
            num_ctx: 4096,
            num_predict: 2048,
            api_key: None,
            parallel: 1,
        }
    }
}
