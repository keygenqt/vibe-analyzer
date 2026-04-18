//! Module containing tests for the TypeScript language parser functionality.
//! These tests verify that TypeScript source code is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests that a TypeScript source file is parsed correctly and matches the expected JSON output.
#[test]
fn test_typescript_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/typescript/sample.ts").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/typescript/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "ts").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
