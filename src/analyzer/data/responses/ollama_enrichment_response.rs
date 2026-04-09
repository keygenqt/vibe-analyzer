//! LLM enrichment response structure for batch file analysis.
//! Contains enriched data for multiple files in a single response.

use serde::Deserialize;
use serde::Serialize;

use crate::analyzer::data::responses::ollama_file_enrichment_response::OllamaFileEnrichmentResponse;

/// Container for batch enrichment responses from Ollama.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaEnrichmentResponse {
    /// Enriched analysis data for each file in the batch
    pub files: Vec<OllamaFileEnrichmentResponse>,
}
