//! Language parsers for AST extraction
//!
//! This module contains tree-sitter based parsers for various programming languages.
//! Each parser implements the [`LanguageParser`] trait and extracts:
//! - Functions, classes, structs, enums, traits
//! - Imports and variables
//! - Comments and doc comments

pub mod arkts;
pub mod bash;
pub mod batch;
pub mod cpp;
pub mod csharp;
pub mod dart;
pub mod go;
pub mod haskell;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod sql;
pub mod swift;
pub mod typescript;
pub mod xml;
