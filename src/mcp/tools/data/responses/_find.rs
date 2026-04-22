//! Response structure for the find_code tool.

use crate::opensearch::data::responses::search_ast::SearchAst;
use crate::utils::serde_helpers::serialize_as_snake_case;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FindResponse {
    /// Original search query.
    pub query: String,
    /// Found AST matches.
    #[serde(serialize_with = "serialize_as_snake_case")]
    pub results: Vec<SearchAst>,
    /// Whether full content was included.
    pub include_content_used: bool,
}

impl FindResponse {
    /// Returns a formatted tool result with JSON data.
    pub fn format(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        format!(
            "[TOOL RESULT - Find: '{}' in all projects]\n{}\n\nUse this data to respond to the user.",
            self.query, json
        )
    }
}
