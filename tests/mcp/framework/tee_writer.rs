//! Tee writer that duplicates output to file and stdout.
//! Used for logging child process output to both terminal and log file.

use std::io::{Stdout, Write};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::Level;

/// Writer that duplicates output to file and stdout.
/// Filters terminal output by log level.
pub struct TeeWriter {
    file: Arc<Mutex<std::fs::File>>,
    stdout: Stdout,
    log_level: Level,
}

impl TeeWriter {
    /// Creates a new TeeWriter from a shared file handle with log level.
    pub fn new(file: Arc<Mutex<std::fs::File>>, log_level: Level) -> Self {
        Self {
            file,
            stdout: std::io::stdout(),
            log_level,
        }
    }

    /// Checks if a line should be printed to terminal based on log level.
    fn should_print(&self, line: &str) -> bool {
        let level = if line.contains("ERROR") && line.contains("[0m") {
            Level::ERROR
        } else if line.contains("WARN") && line.contains("[0m") {
            Level::WARN
        } else if line.contains("INFO") && line.contains("[0m") {
            Level::INFO
        } else if line.contains("DEBUG") && line.contains("[0m") {
            Level::DEBUG
        } else if line.contains("TRACE") && line.contains("[0m") {
            Level::TRACE
        } else {
            // Raw output without ANSI colors (e.g., docker, child processes)
            // Only show at TRACE level
            Level::TRACE
        };

        level <= self.log_level
    }
}

impl Write for TeeWriter {
    /// Writes data to both the log file and stdout (filtered by log level).
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Always write to file
        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(buf);
            let _ = file.flush();
        }

        // Buffer and process line by line for stdout filtering
        let mut line_buffer = Vec::new();
        let mut stdout = self.stdout.lock();

        for &byte in buf {
            line_buffer.push(byte);
            if byte == b'\n' {
                let line = String::from_utf8_lossy(&line_buffer);
                if self.should_print(&line) {
                    let _ = stdout.write_all(&line_buffer);
                }
                line_buffer.clear();
            }
        }

        // Flush remaining partial line (print it since we can't determine level)
        if !line_buffer.is_empty() {
            let _ = stdout.write_all(&line_buffer);
        }

        let _ = stdout.flush();
        Ok(buf.len())
    }

    /// Flushes both the log file and stdout.
    fn flush(&mut self) -> std::io::Result<()> {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
        self.stdout.flush()
    }
}

impl Clone for TeeWriter {
    /// Clones the TeeWriter, sharing the same file handle and log level.
    fn clone(&self) -> Self {
        Self {
            file: Arc::clone(&self.file),
            stdout: std::io::stdout(),
            log_level: self.log_level,
        }
    }
}

impl From<TeeWriter> for Stdio {
    /// Converts a TeeWriter into Stdio by creating a pipe and spawning a background thread
    /// that copies data from the pipe to the TeeWriter.
    fn from(writer: TeeWriter) -> Self {
        let (mut reader, writer_pipe) =
            os_pipe::pipe().expect("Failed to create pipe for TeeWriter");

        thread::spawn(move || {
            let mut tee = writer;
            // Copy all data from pipe to TeeWriter
            // Ignore errors to avoid panicking in background thread
            let _ = std::io::copy(&mut reader, &mut tee);
        });

        Stdio::from(writer_pipe)
    }
}
