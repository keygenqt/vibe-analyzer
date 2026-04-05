//! Abstract Syntax Tree (AST) parsing module
//!
//! This module provides language-agnostic AST parsing infrastructure.
//! It includes:
//! - [`parser`] - Main parsing logic and `AstData` structure
//! - [`traits`] - `LanguageParser` trait for implementing language parsers
//! - [`langs`] - Language-specific parser implementations

pub mod langs;
pub mod parser;
pub mod traits;
