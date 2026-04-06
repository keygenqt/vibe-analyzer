use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

#[test]
fn test_java_parser() {
    let code = fs::read_to_string("tests/fixtures/java/sample.java").unwrap();
    let json = fs::read_to_string("tests/fixtures/java/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    let ast = parse_ast(&code, "java").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    assert_eq!(actual, expected);
}
