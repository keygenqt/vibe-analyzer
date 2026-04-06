//! Module documentation for testing
//! This module contains various Rust constructs

use crate::utils::path;
use std::collections::HashMap;
use std::fs::{self, File};

// This is also ignored

/// Adds two numbers
///
/// # Examples
/// ```
/// let result = add(2, 3);
/// ```
fn add(a: i32, b: i32) -> i32 {
    // internal comment - ignored
    a + b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Async function example
pub async fn fetch_data(url: &str) -> Result<String, std::io::Error> {
    Ok(String::new())
}

/// Unsafe function example
pub unsafe fn dangerous() {
    // unsafe code
}

/// User structure with fields
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    age: u32,
    email: String,
}

/// Internal struct
struct Config {
    debug: bool,
    max_size: usize,
}

/// Color enum
pub enum Color {
    Red,
    Green,
    Blue,
}

/// Result enum with values
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

/// Display trait
pub trait Display {
    fn display(&self) -> String;
}

/// Debug trait
pub trait Debug {
    fn debug(&self);
}

/// Global constants
pub const MAX_SIZE: usize = 1024;
const DEFAULT_TIMEOUT: u64 = 30;

/// Static variables
pub static APP_NAME: &str = "vibe-analyzer";
static mut COUNTER: u32 = 0;

/// Type alias
pub type MyResult<T> = Result<T, String>;

// Regular comment at the end - ignored
