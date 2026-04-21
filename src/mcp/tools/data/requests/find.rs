//! Request parameters for the find_code tool.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Parameters for the find_code tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindParams {
    /// Find query - looks in function names, signatures, comments, and imports.
    pub query: String,
    /// Include full file content in results. Default is false - returns only signatures.
    #[serde(default)]
    pub include_content: bool,
    /// Maximum number of results (default: 1).
    #[serde(default = "FindParams::default_limit")]
    pub limit: usize,
}

impl FindParams {
    pub const fn default_limit() -> usize {
        1
    }

    /// Convert to FindOptions with translate enabled.
    pub fn into_find_options(self, is_keywords: bool) -> FindOptions {
        FindOptions {
            query: self.query,
            include_content: self.include_content,
            limit: self.limit,
            is_keywords,
        }
    }
}

/// Parameters for aliased find tools (find_tests, commit_rules, etc.).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OptionalFindParams {
    /// Find query - optional for aliased tools.
    #[serde(default)]
    pub query: Option<String>,
    /// Include full file content in results. Default is false.
    #[serde(default)]
    pub include_content: bool,
    /// Maximum number of results (default: 2).
    #[serde(default = "FindParams::default_limit")]
    pub limit: usize,
}

impl OptionalFindParams {
    pub fn into_find_options(self, default_query: impl Into<String>) -> FindOptions {
        FindOptions {
            query: self.query.unwrap_or_else(|| default_query.into()),
            include_content: self.include_content,
            limit: self.limit,
            is_keywords: false,
        }
    }
}

/// Internal find options.
pub struct FindOptions {
    pub query: String,
    pub include_content: bool,
    pub limit: usize,
    pub is_keywords: bool,
}
