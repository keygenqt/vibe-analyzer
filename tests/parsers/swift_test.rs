//! Module containing tests for the Swift language parser functionality.
//! These tests verify that Swift source code is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests that a Swift source file is parsed correctly and matches the expected JSON output.
#[test]
fn test_swift_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/swift/sample.swift").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/swift/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "swift").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
