//! Metadata enrichment prompt.
//! Generates summary and tags for search indexing.

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::ollama::client::OllamaClient;

impl OllamaClient {
    /// Build prompt for metadata enrichment: summary and tags.
    pub fn enrich_meta_prompt(files: &[FileLanguageModel]) -> String {
        let template_files: Vec<String> = files
            .iter()
            .map(|f| {
                format!(
                    r#"    {{
      "path": "{}",
      "description": "FILL_DESCRIPTION",
      "tags": ["TAG1", "TAG2", "TAG3", "TAG4", "TAG5"]
    }}"#,
                    f.base_model().path()
                )
            })
            .collect();

        let template = format!(r#"{{"files": [\n{}\n]}}"#, template_files.join(",\n"));

        format!(
            r#"COPY this JSON EXACTLY, replacing only the placeholders.

Placeholders to replace:
- "FILL_DESCRIPTION" → write 2-3 sentences about the file
- "TAG1", "TAG2", "TAG3", "TAG4", "TAG5" → replace with 3-5 specific keywords

STRICT RULES:
- Keep ALL {} objects in the array
- Keep ALL "path" values unchanged
- Do NOT skip any object
- Do NOT change JSON structure

JSON TO COPY AND FILL:
{}"#,
            files.len(),
            template
        )
    }

    /// Parse Metadata enrichment response into model.
    pub fn enrich_meta_parse(
        original: &FileLanguageModel,
        item: &serde_json::Value,
    ) -> FileLanguageModel {
        let mut model = original.clone();
        model.analyze.summary = item["description"].as_str().map(|s| s.to_string());
        let llm_tags: Vec<String> = item["tags"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        model.analyze.tags.extend(llm_tags);
        model
    }
}
