//! Batch request for file-level LLM enrichment.
//! Groups multiple files into a single request respecting max_chunk_chars limit.

use crate::scanner::data::output::file_analysis::FileAnalysis;

#[derive(Debug)]
pub struct OllamaFilesRequest {
    /// Maximum allowed characters per batch
    max_chunk_chars: usize,
    /// Current total characters in the batch
    current_chars: usize,
    /// Original file analysis data for later matching with LLM responses
    pub files: Vec<FileAnalysis>,
    /// JSON-serialized file data ready for LLM prompt
    pub values: Vec<String>,
}

impl OllamaFilesRequest {
    pub fn new(max_chunk_chars: usize) -> Self {
        Self {
            max_chunk_chars,
            current_chars: 0,
            files: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Attempts to add a file to the batch.
    pub fn add(&mut self, file: &FileAnalysis) -> bool {
        let json_content = match serde_json::to_string(file) {
            Ok(json) => json,
            Err(_) => return false,
        };
        let json_chars = json_content.chars().count();
        if self.current_chars + json_chars > self.max_chunk_chars {
            return false;
        }
        self.files.push(file.clone());
        self.values.push(json_content);
        self.current_chars += json_chars;
        true
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
