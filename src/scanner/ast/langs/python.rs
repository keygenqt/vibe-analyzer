//! Python language AST parser

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::scanner::ast::{
    data::{
        ast_data::AstData, ast_data_class::AstDataClass, ast_data_function::AstDataFunction,
        ast_data_variable::AstDataVariable,
    },
    traits::LanguageParser,
};

/// Parser for Python language files
#[derive(Default)]
pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse(&self, content: &str) -> Result<AstData> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_python::LANGUAGE.into())?;

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Ok(AstData::default()),
        };

        let root_node = tree.root_node();
        let mut data = AstData::default();
        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut variables = Vec::new();

        visit_node(
            &root_node,
            content,
            &mut data,
            &mut functions,
            &mut classes,
            &mut variables,
        );

        data.functions = functions;
        data.classes = classes;
        data.variables = variables;

        Ok(data)
    }
}

/// Recursively traverses the AST and extracts relevant nodes
fn visit_node(
    node: &Node,
    content: &str,
    data: &mut AstData,
    functions: &mut Vec<AstDataFunction>,
    classes: &mut Vec<AstDataClass>,
    variables: &mut Vec<AstDataVariable>,
) {
    match node.kind() {
        // Module docstring
        "expression_statement" => {
            if let Some(child) = node.child(0) {
                match child.kind() {
                    // Module docstring
                    "string" => {
                        if data.comments.is_empty() && node.prev_sibling().is_none() {
                            if let Ok(text) = child.utf8_text(content.as_bytes()) {
                                let cleaned = text.trim();
                                if cleaned.starts_with("\"\"\"") || cleaned.starts_with("'''") {
                                    let body = cleaned[3..cleaned.len() - 3].trim().to_string();
                                    if !body.is_empty() {
                                        data.comments.push(body);
                                    }
                                }
                            }
                        }
                    }
                    // Type-annotated variable (MAX_SIZE: int = 1024)
                    "annotated_assignment" => {
                        if !is_inside_class(node) && !is_inside_function(node) {
                            if let Some(signature) =
                                extract_python_variable_signature(&child, content)
                            {
                                variables.push(AstDataVariable {
                                    signature,
                                    comments: vec![],
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Top-level functions
        "function_definition" => {
            if !is_inside_class(node) {
                if let Some(signature) = extract_python_function_signature(node, content) {
                    functions.push(AstDataFunction {
                        signature,
                        comments: extract_python_docstring(node, content),
                    });
                }
            }
        }

        // Classes
        "class_definition" => {
            if let Some(signature) = extract_python_class_signature(node, content) {
                classes.push(AstDataClass {
                    signature,
                    comments: extract_python_docstring(node, content),
                });
            }
        }

        // Global variables
        "assignment" => {
            if !is_inside_class(node) && !is_inside_function(node) {
                if let Some(signature) = extract_python_variable_signature(node, content) {
                    variables.push(AstDataVariable {
                        signature,
                        comments: vec![],
                    });
                }
            }
        }

        // Imports
        "import_statement" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let cleaned = text.replace("import ", "").trim().to_string();
                for part in cleaned.split(',') {
                    let module = part.trim().split('.').next().unwrap_or(part).to_string();
                    if !module.is_empty() && !data.imports.contains(&module) {
                        data.imports.push(module);
                    }
                }
            }
        }
        "import_from_statement" => {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                if let Some(from_pos) = text.find("from ") {
                    let rest = &text[from_pos + 5..];
                    if let Some(import_pos) = rest.find(" import ") {
                        let module = rest[..import_pos].trim().to_string();
                        if !module.is_empty() && !data.imports.contains(&module) {
                            data.imports.push(module);
                        }
                    }
                }
            }
        }

        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_node(&child, content, data, functions, classes, variables);
    }
}

/// Extracts function signature without body and without colon
fn extract_python_function_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().trim_end_matches(':').to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts class signature without colon
fn extract_python_class_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let first_line = text.lines().next().unwrap_or(text);
        let signature = first_line.trim().trim_end_matches(':').to_string();
        if !signature.is_empty() {
            return Some(signature);
        }
    }
    None
}

/// Extracts variable signature without value
fn extract_python_variable_signature(node: &Node, content: &str) -> Option<String> {
    if let Ok(text) = node.utf8_text(content.as_bytes()) {
        let signature = text.split('=').next().unwrap_or(text).trim().to_string();
        if !signature.is_empty() && !signature.contains('(') {
            return Some(signature);
        }
    }
    None
}

/// Extracts docstring from function or class
fn extract_python_docstring(node: &Node, content: &str) -> Vec<String> {
    if let Some(body) = node.child_by_field_name("body") {
        if let Some(expr) = body.child(0) {
            if expr.kind() == "expression_statement" {
                if let Some(string_node) = expr.child(0) {
                    if string_node.kind() == "string" {
                        if let Ok(text) = string_node.utf8_text(content.as_bytes()) {
                            let trimmed = text.trim();
                            if (trimmed.starts_with("\"\"\"") && trimmed.ends_with("\"\"\""))
                                || (trimmed.starts_with("'''") && trimmed.ends_with("'''"))
                            {
                                let inner = trimmed[3..trimmed.len() - 3].trim().to_string();
                                if !inner.is_empty() {
                                    return vec![inner];
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vec![]
}

/// Checks if a node is inside a class
fn is_inside_class(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "class_definition" {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Checks if a node is inside a function
fn is_inside_function(node: &Node) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "function_definition" {
            return true;
        }
        parent = p.parent();
    }
    false
}
