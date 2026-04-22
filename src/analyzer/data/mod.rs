//! Data structures for enriched analysis results.
//! Defines serializable types for LLM requests, responses, and enriched output.

pub mod output {
    pub mod analyzer_file_output;
    pub mod analyzer_project_output;
    pub mod analyzer_projects_output;
}

pub mod requests {
    pub mod analyzer_file_request;
    pub mod analyzer_files_request;
    pub mod analyzer_summary_request;
}

pub mod responses {
    pub mod ollama_enrichment_response;
    pub mod ollama_file_enrichment_response;
    pub mod ollama_generate_response;
    pub mod ollama_model_info_response;
    pub mod ollama_tags_response;
}
