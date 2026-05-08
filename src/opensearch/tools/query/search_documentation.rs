//! Documentation search functionality for OpenSearch indices.

use anyhow::Result;
use serde_json::Value;
use serde_json::json;

use crate::models::file_models::FileModels;
use crate::opensearch::tools::tool_client::OpenSearchToolClient;

/// Detects script types in query string.
struct ScriptFlags {
    /// Cyrillic script detected.
    cyrillic: bool,
    /// Latin script detected.
    latin: bool,
    /// CJK (Chinese/Japanese/Korean) script detected.
    cjk: bool,
}

impl OpenSearchToolClient {
    /// Searches markdown documentation across analysis indexes.
    pub async fn search_documentation(
        &self,
        query: Option<String>,
        limit: usize,
    ) -> Result<Vec<FileModels>> {
        let search_query = match query {
            Some(ref q) => Self::build_docs_query(q, limit),
            None => json!({
                "size": limit,
                "query": {
                    "bool": {
                        "filter": [
                            { "term": { "type": "language" } },
                            { "term": { "language": "Markdown" } }
                        ],
                        "should": [
                            { "exists": { "field": "headings" } },
                            { "exists": { "field": "links" } },
                            { "exists": { "field": "frontmatter" } }
                        ],
                        "minimum_should_match": 1
                    }
                }
            }),
        };

        let resp = self.call(search_query).await?;
        let hits = resp["hits"]["hits"].as_array().unwrap();

        let results: Vec<FileModels> = hits
            .iter()
            .filter_map(|h| serde_json::from_value(h["_source"].clone()).ok())
            .collect();

        Ok(results)
    }

    /// Detects which scripts are present in the query (Cyrillic, Latin, CJK).
    fn detect_scripts(q: &str) -> ScriptFlags {
        ScriptFlags {
            cyrillic: q.chars().any(|c| {
                ('\u{0400}'..='\u{04FF}').contains(&c) || ('\u{0500}'..='\u{052F}').contains(&c)
            }),
            latin: q.chars().any(|c| c.is_alphabetic() && c.is_ascii()),
            cjk: q.chars().any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c)),
        }
    }

    /// Returns filtered words (> 2 chars) from query.
    fn extract_words(q: &str) -> Vec<&str> {
        q.split_whitespace()
            .filter(|w| w.chars().count() > 2)
            .collect()
    }

    /// Builds wildcard matches for title.keyword (including stemmed variant for words > 4 chars).
    fn build_title_wildcards(words: &[&str]) -> Vec<Value> {
        let mut wildcards = Vec::new();
        for word in words {
            wildcards.push(json!({ "wildcard": { "headings.title.keyword": { "value": format!("*{}*", word), "boost": 10.0 } } }));
            let char_count = word.chars().count();
            if char_count > 4 {
                let stemmed: String = word.chars().take(char_count - 1).collect();
                wildcards.push(json!({ "wildcard": { "headings.title.keyword": { "value": format!("*{}*", stemmed), "boost": 10.0 } } }));
            }
        }
        wildcards
    }

    /// Adds language-specific match clauses.
    fn add_language_matches(
        q: &str,
        flags: &ScriptFlags,
        title: &mut Vec<Value>,
        preview: &mut Vec<Value>,
        links: &mut Vec<Value>,
        tags: &mut Vec<Value>,
    ) {
        if flags.cyrillic {
            title.push(json!({ "match": { "headings.title.russian": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } }));
            preview.push(json!({ "match_phrase": { "headings.preview.russian": { "query": q, "boost": 2.0 } } }));
            links.push(json!({ "match": { "links.text.russian": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } }));
            tags.push(json!({ "match": { "tags.russian": { "query": q, "boost": 1.0 } } }));
        }
        if flags.latin {
            title.push(json!({ "match": { "headings.title.english": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } }));
            preview.push(json!({ "match_phrase": { "headings.preview.english": { "query": q, "boost": 2.0 } } }));
            links.push(json!({ "match": { "links.text.english": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } }));
            tags.push(json!({ "match": { "tags.english": { "query": q, "boost": 1.0 } } }));
        }
        if flags.cjk {
            title.push(json!({ "match": { "headings.title.chinese": { "query": q, "operator": "or", "boost": 4.0, "fuzziness": "AUTO" } } }));
            preview.push(json!({ "match_phrase": { "headings.preview.chinese": { "query": q, "boost": 2.0 } } }));
            links.push(json!({ "match": { "links.text.chinese": { "query": q, "operator": "or", "boost": 2.0, "fuzziness": "AUTO" } } }));
            tags.push(json!({ "match": { "tags.chinese": { "query": q, "boost": 1.0 } } }));
        }
    }

    /// Builds the full OpenSearch query JSON.
    fn build_docs_query(q: &str, limit: usize) -> Value {
        let flags = Self::detect_scripts(q);
        let words = Self::extract_words(q);

        let mut title_matches = Self::build_title_wildcards(&words);
        let mut preview_matches = Vec::new();
        let mut link_matches = Vec::new();
        let mut tag_matches = Vec::new();

        Self::add_language_matches(
            q,
            &flags,
            &mut title_matches,
            &mut preview_matches,
            &mut link_matches,
            &mut tag_matches,
        );

        let mut headings_should = vec![];
        headings_should.extend(title_matches);
        headings_should.extend(preview_matches);

        let mut links_should = link_matches;
        if !links_should.is_empty() {
            links_should.push(json!({ "term": { "links.url": { "value": q, "boost": 1.5 } } }));
        }

        let mut should =
            vec![json!({ "term": { "frontmatter.knowledge": { "value": "true", "boost": 5.0 } } })];

        if !headings_should.is_empty() {
            should.push(json!({
                "nested": {
                    "path": "headings",
                    "boost": 10.0,
                    "query": {
                        "bool": {
                            "should": headings_should
                        }
                    }
                }
            }));
        }

        if !links_should.is_empty() {
            should.push(json!({
                "nested": {
                    "path": "links",
                    "query": {
                        "bool": {
                            "should": links_should
                        }
                    }
                }
            }));
        }

        should.push(json!({ "match": { "code_blocks": { "query": q, "boost": 1.5 } } }));
        should.extend(tag_matches);

        json!({
            "size": limit,
            "query": {
                "bool": {
                    "filter": [
                        { "term": { "type": "language" } },
                        { "term": { "language": "Markdown" } }
                    ],
                    "should": should,
                    "minimum_should_match": 1
                }
            }
        })
    }
}
