//! Pretty printing macros for CLI output
//!
//! This module defines macros for formatted console output with colors:
//! - `print_info!`, `print_warning!`, `print_error!`, `print_success!`
//! - `print_progress!` (updates same line)
//! - `exit_info!`, `exit_error!` (print and exit)

/// Pretty print error
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)))
    }};
}

/// Pretty print info
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", format!($($arg)*)))
    }};
}

/// Pretty print warning
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", format!($($arg)*)))
    }};
}

/// Pretty print success
#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", format!($($arg)*)))
    }};
}

/// Pretty print state
#[macro_export]
macro_rules! print_state {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", format!($($arg)*)))
    }};
}

/// Print info and exit
#[macro_export]
macro_rules! exit_info {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", format!($($arg)*)));
        std::process::exit(1);
    }};
}

/// Pretty print progress (same line, overwrites)
#[macro_export]
macro_rules! print_progress {
    ($($arg:tt)*) => {{
        print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}", format!($($arg)*));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }};
}

/// Print error and exit
#[macro_export]
macro_rules! exit_error {
    ($($arg:tt)*) => {{
        println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)));
        std::process::exit(1);
    }};
}
