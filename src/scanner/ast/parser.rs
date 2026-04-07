//! AST parsing infrastructure and data structures

use crate::scanner::ast::{
    data::ast_data::AstData,
    langs::{
        arkts::ArkTSParser, bash::BashParser, batch::BatchParser, csharp::CSharpParser,
        dart::DartParser, go::GoParser, java::JavaParser, javascript::JavaScriptParser,
        kotlin::KotlinParser, python::PythonParser, rust::RustParser, swift::SwiftParser,
        typescript::TypeScriptParser,
    },
};

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

    // // Multi-extension languages
    register_parser::<KotlinParser>(&mut map, &["kt", "kts"]);
    register_parser::<JavaScriptParser>(&mut map, &["js", "jsx"]);
    register_parser::<TypeScriptParser>(&mut map, &["ts", "tsx"]);
    register_parser::<ArkTSParser>(&mut map, &["ets", "arkts"]);
    register_parser::<BashParser>(&mut map, &["sh", "bash", "zsh"]);
    register_parser::<BatchParser>(&mut map, &["bat", "cmd"]);

    map
});

/// Parses source code and returns AST data for the specified language
pub fn parse_ast(content: &str, extension: &str) -> Result<Option<AstData>> {
    let Some(parser) = PARSERS.get(extension) else {
        return Ok(None);
    };
    parser.parse(content).map(Some)
}
