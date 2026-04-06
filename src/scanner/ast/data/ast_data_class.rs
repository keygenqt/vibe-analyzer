//! AST data structures for class definitions

use serde::{Deserialize, Serialize};

/// Container for class-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataClass {
    /// Full class signature (e.g., "class UserService extends BaseService")
    pub signature: String,

    /// Documentation comments and regular comments associated with the class
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
