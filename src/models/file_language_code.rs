//! File language code model.
//! Code structures extracted from source files.

use serde::Deserialize;
use serde::Serialize;

use crate::models::lang_class::LangClassModel;
use crate::models::lang_enum::LangEnumModel;
use crate::models::lang_function::LangFunctionModel;
use crate::models::lang_interface::LangInterfaceModel;
use crate::models::lang_struct::LangStructModel;
use crate::models::lang_variable::LangVariableModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Code structures extracted from source files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageCodeModel {
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub header_comments: Vec<String>,
    /// Import declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,
    /// Top-level functions and methods.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<LangFunctionModel>,
    /// Class declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub classes: Vec<LangClassModel>,
    /// Struct declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub structs: Vec<LangStructModel>,
    /// Enum declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub enums: Vec<LangEnumModel>,
    /// Interface or trait declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub interfaces: Vec<LangInterfaceModel>,
    /// Global variables and constants.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variables: Vec<LangVariableModel>,
}

/// Format implementation for file language code model.
impl TraitFormatModel for FileLanguageCodeModel {
    fn format_ast(&self) -> Self {
        Self {
            header_comments: self.header_comments.clone(),
            imports: self.imports.clone(),
            functions: self.functions.iter().map(|f| f.format_ast()).collect(),
            classes: self.classes.iter().map(|c| c.format_ast()).collect(),
            structs: self.structs.iter().map(|s| s.format_ast()).collect(),
            enums: self.enums.iter().map(|e| e.format_ast()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_ast()).collect(),
            variables: self.variables.iter().map(|v| v.format_ast()).collect(),
        }
    }

    fn format_analyze(&self) -> Self {
        Self {
            header_comments: self.header_comments.clone(),
            imports: self.imports.clone(),
            functions: self.functions.iter().map(|f| f.format_analyze()).collect(),
            classes: self.classes.iter().map(|c| c.format_analyze()).collect(),
            structs: self.structs.iter().map(|s| s.format_analyze()).collect(),
            enums: self.enums.iter().map(|e| e.format_analyze()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_analyze()).collect(),
            variables: self.variables.iter().map(|v| v.format_analyze()).collect(),
        }
    }

    fn format_index(&self) -> Self {
        Self {
            header_comments: self.header_comments.clone(),
            imports: self.imports.clone(),
            functions: self.functions.iter().map(|f| f.format_index()).collect(),
            classes: self.classes.iter().map(|c| c.format_index()).collect(),
            structs: self.structs.iter().map(|s| s.format_index()).collect(),
            enums: self.enums.iter().map(|e| e.format_index()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_index()).collect(),
            variables: self.variables.iter().map(|v| v.format_index()).collect(),
        }
    }

    fn format_tool(&self) -> Self {
        Self {
            header_comments: self.header_comments.clone(),
            imports: self.imports.clone(),
            functions: self.functions.iter().map(|f| f.format_tool()).collect(),
            classes: self.classes.iter().map(|c| c.format_tool()).collect(),
            structs: self.structs.iter().map(|s| s.format_tool()).collect(),
            enums: self.enums.iter().map(|e| e.format_tool()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_tool()).collect(),
            variables: self.variables.iter().map(|v| v.format_tool()).collect(),
        }
    }
}
