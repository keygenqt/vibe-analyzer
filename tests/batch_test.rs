//! Unit tests for the Windows Batch language AST parser.
//! Verifies that parsing a Batch fixture file produces the expected JSON snapshot.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests Batch parser against a fixture file with expected JSON snapshot.
#[test]
fn test_batch_parser() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/batch/sample.bat").unwrap();
    let json = fs::read_to_string("tests/fixtures/batch/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "bat").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
