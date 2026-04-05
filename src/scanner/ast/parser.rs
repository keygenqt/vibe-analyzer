//! AST parsing infrastructure and data structures
//!
//! This module provides:
//! - [`AstData`] - Container for extracted AST information (functions, classes, imports, etc.)
//! - Language parser registry mapping file extensions to parsers
//! - [`parse_ast`] - Main entry point for parsing source code

use super::langs::arkts::ArkTSParser;
use super::langs::bash::BashParser;
use super::langs::batch::BatchParser;
use super::langs::cpp::CppParser;
use super::langs::csharp::CSharpParser;
use super::langs::dart::DartParser;
use super::langs::go::GoParser;
use super::langs::haskell::HaskellParser;
use super::langs::java::JavaParser;
use super::langs::javascript::JavaScriptParser;
use super::langs::kotlin::KotlinParser;
use super::langs::php::PhpParser;
use super::langs::python::PythonParser;
use super::langs::ruby::RubyParser;
use super::langs::rust::RustParser;
use super::langs::sql::SqlParser;
use super::langs::swift::SwiftParser;
use super::langs::typescript::TypeScriptParser;
use super::langs::xml::XmlParser;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::traits::LanguageParser;

/// Abstract Syntax Tree data extracted from source files
///
/// Contains various code elements extracted by language parsers.
/// Empty fields are automatically omitted during JSON serialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstData {
    /// Function/method names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<String>,

    /// Class names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<String>,

    /// Import/use/include statements
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    /// Struct/record names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub structs: Vec<String>,

    /// Enum names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<String>,

    /// Trait/interface/protocol names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<String>,

    /// Database table names (SQL)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,

    /// Database view names (SQL)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<String>,

    /// Regular comments
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,

    /// Documentation comments (///, /**), docstrings
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc_comments: Vec<String>,

    /// Variable names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<String>,

    /// XML/HTML tag names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// XML/HTML attribute names
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
}

/// Registers a language parser for multiple file extensions
///
/// # Arguments
/// * `map` - HashMap to register the parser in
/// * `extensions` - Slice of file extensions (e.g., &["rs", "rsx"])
fn register_parser<P: LanguageParser + Default + 'static>(
    map: &mut HashMap<String, Box<dyn LanguageParser>>,
    extensions: &[&str],
) {
    for ext in extensions {
        map.insert(ext.to_string(), Box::new(P::default()));
    }
}

/// Static registry mapping file extensions to their language parsers
static PARSERS: Lazy<HashMap<String, Box<dyn LanguageParser>>> = Lazy::new(|| {
    let mut map: HashMap<String, Box<dyn LanguageParser>> = HashMap::new();

    // Single-extension languages
    register_parser::<RustParser>(&mut map, &["rs"]);
    register_parser::<PythonParser>(&mut map, &["py"]);
    register_parser::<GoParser>(&mut map, &["go"]);
    register_parser::<JavaParser>(&mut map, &["java"]);
    register_parser::<CSharpParser>(&mut map, &["cs"]);
    register_parser::<SwiftParser>(&mut map, &["swift"]);
    register_parser::<PhpParser>(&mut map, &["php"]);
    register_parser::<RubyParser>(&mut map, &["rb"]);
    register_parser::<HaskellParser>(&mut map, &["hs"]);
    register_parser::<DartParser>(&mut map, &["dart"]);
    register_parser::<SqlParser>(&mut map, &["sql"]);

    // Multi-extension languages
    register_parser::<KotlinParser>(&mut map, &["kt", "kts"]);
    register_parser::<JavaScriptParser>(&mut map, &["js", "jsx"]);
    register_parser::<TypeScriptParser>(&mut map, &["ts", "tsx"]);
    register_parser::<CppParser>(&mut map, &["cpp", "hpp", "cc", "cxx"]);
    register_parser::<ArkTSParser>(&mut map, &["ets", "arkts"]);
    register_parser::<BashParser>(&mut map, &["sh", "bash", "zsh"]);
    register_parser::<BatchParser>(&mut map, &["bat", "cmd"]);
    register_parser::<XmlParser>(&mut map, &["xml", "ui", "qrc", "xaml", "glade"]);

    map
});

/// Parses source code and returns AST data for the specified language
///
/// Returns `None` if no parser is registered for this extension.
///
/// # Arguments
/// * `content` - Source code content as string
/// * `extension` - File extension (e.g., "rs", "py") to determine parser
///
/// # Returns
/// * `Ok(Some(AstData))` - Successfully parsed AST data
/// * `Ok(None)` - No parser found for this extension (not an error)
/// * `Err` - Parser error (invalid syntax, etc.)
pub fn parse_ast(content: &str, extension: &str) -> Result<Option<AstData>> {
    let Some(parser) = PARSERS.get(extension) else {
        return Ok(None);
    };
    parser.parse(content).map(Some)
}
