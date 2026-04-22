//! Batch request for file-level LLM enrichment.
//! Groups multiple files into a single request respecting max_chunk_chars limit.

use serde::Deserialize;
use serde::Serialize;

use crate::configs::ollama::OllamaConfig;
use crate::scanner::ast::data::ast_data_class::AstDataClass;
use crate::scanner::ast::data::ast_data_enum::AstDataEnum;
use crate::scanner::ast::data::ast_data_function::AstDataFunction;
use crate::scanner::ast::data::ast_data_interface::AstDataInterface;
use crate::scanner::ast::data::ast_data_struct::AstDataStruct;
use crate::scanner::ast::data::ast_data_variable::AstDataVariable;
use crate::scanner::data::output::scan_file_analysis_output::ScanFileAnalysisOutput;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "File")]
pub struct AnalyzerFileRequest {
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

impl AnalyzerFileRequest {
    pub fn new(config: &OllamaConfig, file: &ScanFileAnalysisOutput) -> Self {
        let ast = file.ast.as_ref();
        Self {
            path: file.path.clone(),
            extension: file.extension.clone(),
            header_comments: ast.map(|a| a.header_comments.clone()).unwrap_or_default(),
            classes: ast.map(|a| a.classes.clone()).unwrap_or_default(),
            structs: ast.map(|a| a.structs.clone()).unwrap_or_default(),
            imports: if config.ast_imports {
                ast.map(|a| a.imports.clone()).unwrap_or_default()
            } else {
                Vec::new()
            },
            variables: if config.ast_variables {
                ast.map(|a| a.variables.clone()).unwrap_or_default()
            } else {
                Vec::new()
            },
            functions: if config.ast_functions {
                ast.map(|a| a.functions.clone()).unwrap_or_default()
            } else {
                Vec::new()
            },
            enums: if config.ast_enums {
                ast.map(|a| a.enums.clone()).unwrap_or_default()
            } else {
                Vec::new()
            },
            interfaces: if config.ast_interfaces {
                ast.map(|a| a.interfaces.clone()).unwrap_or_default()
            } else {
                Vec::new()
            },
        }
    }
}
