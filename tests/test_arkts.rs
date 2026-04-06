use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

#[test]
fn test_arkts_parser() {
    // Arrange
    let code = fs::read_to_string("tests/fixtures/arkts/sample.ets").unwrap();
    let json = fs::read_to_string("tests/fixtures/arkts/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Act
    let ast = parse_ast(&code, "ets").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    // Assert
    assert_eq!(actual, expected);
}
