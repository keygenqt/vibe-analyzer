//! Metadata enrichment prompt and response parsing.
//! Generates summary and tags for search indexing.

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

use crate::models::file_language::FileLanguageModel;
use crate::models::trait_base_model::TraitBaseModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::utils::normalize::tags_normalize;

pub struct MetaPrompt;

impl MetaPrompt {
    /// Build prompt for metadata enrichment: summary and tags.
    /// Accepts multiple files to batch into one prompt.
    pub fn build(files: &[FileLanguageModel]) -> Result<String> {
        #[derive(Serialize)]
        struct DataPrompt<'a> {
            #[serde(rename = "PATH")]
            path: String,
            #[serde(rename = "DESCRIPTION")]
            description: &'a str,
            #[serde(rename = "TAGS")]
            tags: Vec<&'a str>,
        }

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

RULES:
- Language: English only.

JSON TO COPY AND FILL:
{}

FILE CONTENTS:
{}
"#,
            template_prompt, data_prompt,
        ))
    }

    /// Parse metadata enrichment response into models.
    pub fn parse(originals: &[FileLanguageModel], item: &Value) -> Vec<FileLanguageModel> {
        let items = item.as_array().map(|a| a.as_slice()).unwrap_or(&[]);
        originals
            .iter()
            .map(|original| {
                let mut model = original.clone();
                if let Some(matched) = items
                    .iter()
                    .find(|i| i["PATH"].as_str() == Some(&original.base_model().path()))
                {
                    model.analyze.summary = matched["DESCRIPTION"].as_str().map(|s| s.to_string());
                    let llm_tags: Vec<String> = matched["TAGS"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    model.analyze.tags.extend(tags_normalize(llm_tags));
                }
                model
            })
            .collect()
    }
}
