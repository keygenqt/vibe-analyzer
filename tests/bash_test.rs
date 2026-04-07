//! Unit tests for the Bash/Shell language AST parser.
//! Verifies that parsing a Bash fixture file produces the expected JSON snapshot.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests Bash parser against a fixture file with expected JSON snapshot.
#[test]
fn test_bash_parser() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/bash/sample.sh").unwrap();
    let json = fs::read_to_string("tests/fixtures/bash/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "sh").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
