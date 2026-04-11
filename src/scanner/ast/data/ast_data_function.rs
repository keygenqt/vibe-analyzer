//! AST data structures for function/method definitions.
//! Stores function signatures and associated documentation comments.

use serde::Deserialize;
use serde::Serialize;

/// Container for function-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AstDataFunction {
    /// Full function signature (e.g., "fn add_user(user: User, validate: bool) -> Result<UserId, DbError>")
    pub signature: String,

    /// Documentation comments and regular comments associated with the function
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}
