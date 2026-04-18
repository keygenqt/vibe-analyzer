//! Unit tests for the C# language AST parser.
//! Verifies that parsing a C# fixture file produces the expected JSON snapshot.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests C# parser against a fixture file with expected JSON snapshot.
#[test]
fn test_csharp_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/csharp/sample.cs").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/csharp/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "cs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
