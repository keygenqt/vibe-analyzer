//! Module containing tests for the Markdown parser functionality.
//! These tests verify that Markdown source is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests that a Markdown file with multiple sections is parsed correctly.
#[test]
fn test_python_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/markdown/sample.md").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/markdown/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "md").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
