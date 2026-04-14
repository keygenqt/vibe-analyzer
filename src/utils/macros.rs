//! Pretty printing macros for CLI and server output.
//! Automatically switches between colored console and tracing logs based on runtime mode.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/// Global flag indicating whether running in server mode.
pub static IS_SERVER: AtomicBool = AtomicBool::new(false);

/// Enable server mode (use tracing instead of colored println).
pub fn set_server_mode(enabled: bool) {
    IS_SERVER.store(enabled, Ordering::Relaxed);
}

/// Strip ANSI color codes from a string.
pub fn strip_ansi(s: &str) -> String {
    s.replace("\x1b[1;34m", "")
        .replace("\x1b[1;32m", "")
        .replace("\x1b[1;91m", "")
        .replace("\x1b[1;93m", "")
        .replace("\x1b[1;36m", "")
        .replace("\x1b[0m", "")
        .replace("\x1b[1m", "")
        .replace("\x1b[94m", "")
}

/// Print newline - CLI only, ignored in server mode.
#[macro_export]
macro_rules! print_newline {
    () => {{
        if !$crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            println!();
        }
    }};
}

/// Pretty print error
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::error!("{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Pretty print info
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::info!("{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Pretty print warning
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::warn!("{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Pretty print success
#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::info!(target: "success", "{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Pretty print state
#[macro_export]
macro_rules! print_state {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::debug!("{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Pretty print progress (same line, overwrites) - CLI only, ignored in server mode.
#[macro_export]
macro_rules! print_progress {
    ($($arg:tt)*) => {{
        if !$crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}", format!($($arg)*));
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }};
}

/// Print info and exit - exits in CLI mode, panics in server mode.
#[macro_export]
macro_rules! exit_info {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::info!("{}", clean);
            panic!("{}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", format!($($arg)*)));
            std::process::exit(1);
        }
    }};
}

/// Print error and exit - exits in CLI mode, panics in server mode.
#[macro_export]
macro_rules! exit_error {
    ($($arg:tt)*) => {{
        if $crate::utils::macros::IS_SERVER.load(std::sync::atomic::Ordering::Relaxed) {
            let msg = format!($($arg)*);
            let clean = $crate::utils::macros::strip_ansi(&msg);
            tracing::error!("{}", clean);
            panic!("Fatal error: {}", clean);
        } else {
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)));
            std::process::exit(1);
        }
    }};
}
