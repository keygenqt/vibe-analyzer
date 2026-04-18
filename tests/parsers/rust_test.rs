//! Module containing tests for the Rust language parser functionality.
//! These tests verify that Rust source code is correctly parsed into an AST and matches expected JSON snapshots.

use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

/// Tests that a Rust source file (sample.rs) is parsed correctly and matches the expected JSON output.
#[test]
fn test_rust_parser() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/rust/sample.rs").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/rust/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}

/// Tests that a Rust source file (sample2.rs) is parsed correctly and matches the expected JSON output.
#[test]
fn test_rust_parser2() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/rust/sample2.rs").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/rust/sample2.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}

/// Tests that a Rust source file (sample3.rs) is parsed correctly and matches the expected JSON output.
#[test]
fn test_rust_parser3() {
    // Arrange
    let code = fs::read_to_string("tests/parsers/fixtures/rust/sample3.rs").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/rust/sample3.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
