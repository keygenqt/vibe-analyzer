//! Module containing tests for the Go language parser functionality.
//! These tests verify that Go source code is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests that a Go source file is parsed correctly and matches the expected JSON output.
#[test]
fn test_go_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/go/sample.go").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/go/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "go").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
