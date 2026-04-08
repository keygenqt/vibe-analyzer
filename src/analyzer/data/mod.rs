//! Data structures for enriched analysis results.
//! Defines serializable types for LLM requests, responses, and enriched output.

pub mod output {
    pub mod enriched_file_analysis;
    pub mod enriched_project_analysis;
    pub mod enriched_projects_analysis;
}

pub mod requests {
    pub mod ollama_file_request;
    pub mod ollama_summary_request;
}

pub mod responses {
    pub mod ollama_enrichment_response;
    pub mod ollama_file_enrichment_response;
    pub mod ollama_generate_response;
    pub mod ollama_model_info_response;
    pub mod ollama_tags_response;
}
