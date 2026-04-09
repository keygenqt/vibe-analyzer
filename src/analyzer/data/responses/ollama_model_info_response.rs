//! Ollama model information response structure.
//! Contains metadata about installed models.

use serde::Deserialize;
use serde::Serialize;

/// Individual model information from Ollama /api/tags endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaModelInfoResponse {
    /// Name of the installed model
    pub name: String,
}
