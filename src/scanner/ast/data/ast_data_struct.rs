//! AST data structures for struct/record definitions

use serde::{Deserialize, Serialize};

/// Container for struct-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataStruct {
    /// Full struct signature (e.g., "struct ScanOptions { path: Option<PathBuf> }")
    pub signature: String,

    /// Documentation comments and regular comments associated with the struct
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
