//! AST data structures for interface/protocol definitions.
//! Stores interface signatures and associated documentation comments.

use serde::Deserialize;
use serde::Serialize;

/// Container for interface-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataInterface {
    /// Full interface signature (e.g., "trait Display { fn fmt(&self) -> String }")
    pub signature: String,

    /// Documentation comments and regular comments associated with the interface
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}
