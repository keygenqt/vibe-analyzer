use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

#[test]
fn test_swift_parser() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/swift/sample.swift").unwrap();
    let json = fs::read_to_string("tests/fixtures/swift/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "swift").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
