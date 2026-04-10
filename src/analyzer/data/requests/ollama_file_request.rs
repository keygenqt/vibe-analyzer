//! Batch request for file-level LLM enrichment.
//! Groups multiple files into a single request respecting max_chunk_chars limit.

use serde::Deserialize;
use serde::Serialize;

use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::data::output::file_analysis::FileAnalysis;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OllamaFileRequest {
    /// Relative path from source root
    pub path: String,

    /// File extension without dot (e.g., "rs", "toml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,

    /// Top-level documentation comments at the beginning of the file
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub header_comments: Vec<String>,

    /// Import/use/include statements
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    /// Variables with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<AstDataVariable>,

    /// Functions with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<AstDataFunction>,

    /// Classes with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<AstDataClass>,

    /// Structs with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<AstDataStruct>,

    /// Enums with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<AstDataEnum>,

    /// Interfaces/Protocols/Traits with signatures and comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<AstDataInterface>,
}

impl OllamaFileRequest {
    pub fn new(file: &FileAnalysis) -> Self {
        Self {
            path: file.path.clone(),
            extension: file.extension.clone(),
            header_comments: file
                .ast
                .as_ref()
                .map(|ast| ast.header_comments.clone())
                .unwrap_or_default(),
            imports: file
                .ast
                .as_ref()
                .map(|ast| ast.imports.clone())
                .unwrap_or_default(),
            variables: file
                .ast
                .as_ref()
                .map(|ast| ast.variables.clone())
                .unwrap_or_default(),
            functions: file
                .ast
                .as_ref()
                .map(|ast| ast.functions.clone())
                .unwrap_or_default(),
            classes: file
                .ast
                .as_ref()
                .map(|ast| ast.classes.clone())
                .unwrap_or_default(),
            structs: file
                .ast
                .as_ref()
                .map(|ast| ast.structs.clone())
                .unwrap_or_default(),
            enums: file
                .ast
                .as_ref()
                .map(|ast| ast.enums.clone())
                .unwrap_or_default(),
            interfaces: file
                .ast
                .as_ref()
                .map(|ast| ast.interfaces.clone())
                .unwrap_or_default(),
        }
    }
}
