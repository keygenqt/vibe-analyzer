//! AST parsing infrastructure and data structures.
//! Maintains a registry of language parsers and dispatches parsing requests by file extension.

use crate::scanner::ast::data::ast_data::AstData;
use crate::scanner::ast::langs::arkts::ArkTsParser;
use crate::scanner::ast::langs::bash::BashParser;
use crate::scanner::ast::langs::batch::BatchParser;
use crate::scanner::ast::langs::csharp::CSharpParser;
use crate::scanner::ast::langs::dart::DartParser;
use crate::scanner::ast::langs::go::GoParser;
use crate::scanner::ast::langs::java::JavaParser;
use crate::scanner::ast::langs::javascript::JavaScriptParser;
use crate::scanner::ast::langs::kotlin::KotlinParser;
use crate::scanner::ast::langs::markdown::MarkdownParser;
use crate::scanner::ast::langs::python::PythonParser;
use crate::scanner::ast::langs::rust::RustParser;
use crate::scanner::ast::langs::swift::SwiftParser;
use crate::scanner::ast::langs::typescript::TypeScriptParser;

use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::traits::LanguageParser;

/// Registers a language parser for multiple file extensions
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
    register_parser::<DartParser>(&mut map, &["dart"]);

    // Multi-extension languages
    register_parser::<KotlinParser>(&mut map, &["kt", "kts"]);
    register_parser::<JavaScriptParser>(&mut map, &["js", "jsx"]);
    register_parser::<TypeScriptParser>(&mut map, &["ts", "tsx"]);
    register_parser::<ArkTsParser>(&mut map, &["ets", "arkts"]);
    register_parser::<BashParser>(&mut map, &["sh", "bash", "zsh"]);
    register_parser::<BatchParser>(&mut map, &["bat", "cmd"]);
    register_parser::<MarkdownParser>(&mut map, &["md", "markdown"]);

    map
});

/// Returns the language name for a given file extension.
pub fn get_language_name(extension: &str) -> Option<&'static str> {
    PARSERS.get(extension).map(|p| p.language_name())
}

/// Parses source code and returns AST data for the specified language
pub fn parse_ast(content: &str, extension: &str) -> Result<Option<AstData>> {
    let Some(parser) = PARSERS.get(extension) else {
        return Ok(None);
    };
    parser.parse(content).map(Some)
}
