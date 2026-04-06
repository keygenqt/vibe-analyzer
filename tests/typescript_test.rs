use std::fs;
use vibe_analyzer::scanner::ast::parser::parse_ast;

#[test]
fn test_typescript_parser() {
    let code = fs::read_to_string("tests/fixtures/typescript/sample.ts").unwrap();
    let json = fs::read_to_string("tests/fixtures/typescript/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    let ast = parse_ast(&code, "ts").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    assert_eq!(actual, expected);
}
