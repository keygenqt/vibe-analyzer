//! Simple math library with error handling
//!
//! Provides basic arithmetic operations and calculator struct.

use thiserror::Error;

/// Custom error type for math operations
#[derive(Error, Debug)]
pub enum MathError {
    #[error("division by zero")]
    DivisionByZero,
    #[error("value out of range: {0}")]
    OutOfRange(String),
}

/// Calculates sum of two numbers
///
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calculates product of two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers safely
///
/// # Errors
/// Returns MathError::DivisionByZero if divisor is zero
pub fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

/// User structure
pub struct User {
    name: String,
    age: u8,
}

impl User {
    /// Creates a new user
    ///
    /// # Panics
    /// Panics if age > 150
    pub fn new(name: String, age: u8) -> Self {
        assert!(age <= 150, "Unrealistic age");
        Self { name, age }
    }

    /// Returns user name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns user age
    pub fn age(&self) -> u8 {
        self.age
    }
}

/// Calculator with state
#[derive(Default)]
pub struct Calculator {
    value: i32,
}

impl Calculator {
    /// Creates new calculator with zero value
    pub fn new() -> Self {
        Self { value: 0 }
    }

    /// Adds number to current value
    pub fn add(&mut self, x: i32) {
        self.value = add(self.value, x);
    }

    /// Multiplies current value by number
    pub fn multiply(&mut self, x: i32) {
        self.value = multiply(self.value, x);
    }

    /// Returns current value
    pub fn value(&self) -> i32 {
        self.value
    }
}

/// Function that uses add
pub fn calculate_sum() -> i32 {
    let x = add(10, 20);
    let y = add(5, 15);
    add(x, y)
}

/// Function that uses Calculator
pub fn use_calculator() -> i32 {
    let mut calc = Calculator::new();
    calc.add(5);
    calc.add(3);
    calc.multiply(2);
    calc.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn divide_by_zero_fails() {
        let result = divide(10, 0);
        assert!(result.is_err());
    }

    #[test]
    fn calculator_works() {
        let mut calc = Calculator::new();
        calc.add(10);
        calc.multiply(2);
        assert_eq!(calc.value(), 20);
    }
}
