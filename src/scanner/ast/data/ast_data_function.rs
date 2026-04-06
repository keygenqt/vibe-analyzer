//! AST data structures for function/method definitions

use serde::{Deserialize, Serialize};

/// Container for function-related AST data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstDataFunction {
    /// Full function signature (e.g., "fn add_user(user: User, validate: bool) -> Result<UserId, DbError>")
    pub signature: String,

    /// Documentation comments and regular comments associated with the function
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
}
