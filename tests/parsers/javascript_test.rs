//! Module containing tests for the JavaScript language parser functionality.
//! These tests verify that JavaScript source code is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests JavaScript parser against a fixture file with expected JSON snapshot.
#[test]
fn test_javascript_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/javascript/sample.js").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/javascript/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "js").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
