//! Main AST data structure

use super::ast_data_class::AstDataClass;
use super::ast_data_enum::AstDataEnum;
use super::ast_data_function::AstDataFunction;
use super::ast_data_interface::AstDataInterface;
use super::ast_data_struct::AstDataStruct;
use super::ast_data_variable::AstDataVariable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstData {
    /// Functions with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<AstDataFunction>,

    /// Classes with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<AstDataClass>,

    /// Import/use/include statements
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    /// Structs with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<AstDataStruct>,

    /// Enums with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<AstDataEnum>,

    /// Interfaces/Protocols/Traits with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<AstDataInterface>,

    /// File-level comments and global comments not attached to specific code elements
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,

    /// Variables with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<AstDataVariable>,
}
