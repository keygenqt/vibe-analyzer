//! Batch request for file-level LLM enrichment.
//! Groups multiple files into a single request respecting max_chunk_chars limit.

use crate::analyzer::data::requests::analyzer_file_request::AnalyzerFileRequest;
use crate::configs::ollama::OllamaConfig;
use crate::scanner::data::output::scan_file_analysis_output::ScanFileAnalysisOutput;

#[derive(Debug, Default)]
pub struct AnalyzerFilesRequest {
    /// Current total characters in the batch
    current_chars: usize,
    /// Original file requests for later matching with LLM responses
    pub files: Vec<ScanFileAnalysisOutput>,
    /// JSON-serialized file data ready for LLM prompt
    pub values: Vec<String>,
}

impl AnalyzerFilesRequest {
    pub fn new() -> Self {
        Self {
            current_chars: 0,
            files: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Attempts to add a file to the batch.
    pub fn add(&mut self, file: &ScanFileAnalysisOutput, config: &OllamaConfig) -> bool {
        let file_request = AnalyzerFileRequest::new(config, &file);
        let json_content = match serde_json::to_string(&file_request) {
            Ok(json) => json,
            Err(_) => return false,
        };
        let json_chars = json_content.chars().count();
        if self.current_chars + json_chars > config.max_chunk_chars {
            return false;
        }
        self.files.push(file.clone());
        self.values.push(json_content);
        self.current_chars += json_chars;
        true
    }

    /// Checks if a file can fit into the current batch without adding it.
    pub fn can_fit(&self, file: &ScanFileAnalysisOutput, config: &OllamaConfig) -> bool {
        let file_request = AnalyzerFileRequest::new(config, &file);
        let json_content = match serde_json::to_string(&file_request) {
            Ok(json) => json,
            Err(_) => return false,
        };
        let json_chars = json_content.chars().count();
        if self.files.is_empty() {
            return true;
        }
        self.current_chars + json_chars <= config.max_chunk_chars
    }

    /// Returns true if batch contains no files.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Clears all files from the batch and resets character count.
    pub fn clear(&mut self) {
        self.current_chars = 0;
        self.files.clear();
        self.values.clear();
    }
}
