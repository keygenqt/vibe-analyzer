//! AST data structures for variable definitions

use serde::{Deserialize, Serialize};

/// Container for variable-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataVariable {
    /// Full variable signature (e.g., "let config: AppConfig" or "const MAX_SIZE: usize = 1024")
    pub signature: String,

    /// Documentation comments and regular comments associated with the variable
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
