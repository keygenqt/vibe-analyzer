//! AST data structures for interface/protocol definitions

use serde::{Deserialize, Serialize};

/// Container for interface-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataInterface {
    /// Full interface signature (e.g., "trait Display { fn fmt(&self) -> String }")
    pub signature: String,

    /// Documentation comments and regular comments associated with the interface
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
