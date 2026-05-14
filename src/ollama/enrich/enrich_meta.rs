//! Metadata enrichment prompt.
//! Generates summary and tags for search indexing.

use anyhow::Result;
use serde::Serialize;

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;
use crate::utils::normalize::tags_normalize;

impl OllamaClient {
    /// Build prompt for metadata enrichment: summary and tags.
    pub fn enrich_meta_prompt(files: &[FileLanguageModel]) -> Result<String> {
        // Template for JSON structure that model must fill
        #[derive(Serialize)]
        struct DataPrompt<'a> {
            #[serde(rename = "PATH")]
            path: String,
            #[serde(rename = "DESCRIPTION")]
            description: &'a str,
            #[serde(rename = "TAGS")]
            tags: Vec<&'a str>,
        }

        // Build template with placeholder values
        let template_prompt = serde_json::to_string(
            &files
                .iter()
                .map(|f| DataPrompt {
                    path: f.base_model().path().to_string(),
                    description: "FILL_DESCRIPTION",
                    tags: vec!["TAG1", "TAG2", "TAG3", "TAG4", "TAG5"],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap_or_default();

        let data_prompt = serde_json::to_string(files).unwrap_or_default();

        Ok(format!(
            r#"Return JSON array with only PATH, DESCRIPTION, TAGS fields.

Use data field for analysis. Based on it fill:
- DESCRIPTION — 2-3 sentences about the file
- TAGS — 3-5 keywords (words only, not sentences)

JSON TO COPY AND FILL:
{}

FILE CONTENTS:
{}
"#,
            template_prompt, data_prompt,
        ))
    }

    /// Parse Metadata enrichment response into model.
    pub fn enrich_meta_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.summary = item["DESCRIPTION"].as_str().map(|s| s.to_string());
        let llm_tags: Vec<String> = item["TAGS"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        model.analyze.tags.extend(tags_normalize(llm_tags));
        model
    }
}
