//! AST data structures for enum definitions

use serde::{Deserialize, Serialize};

/// Container for enum-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataEnum {
    /// Full enum signature (e.g., "enum Color { Red, Green, Blue }")
    pub signature: String,

    /// Documentation comments and regular comments associated with the enum
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
