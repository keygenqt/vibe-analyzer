use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

#[test]
fn test_rust_parser() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/rust/sample.rs").unwrap();
    let json = fs::read_to_string("tests/fixtures/rust/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}

#[test]
fn test_rust_parser2() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/rust/sample2.rs").unwrap();
    let json = fs::read_to_string("tests/fixtures/rust/sample2.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}

#[test]
fn test_rust_parser3() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/rust/sample3.rs").unwrap();
    let json = fs::read_to_string("tests/fixtures/rust/sample3.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
