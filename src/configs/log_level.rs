use serde::Deserialize;
use serde::Serialize;

/// Log level for tracing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// CLI mode (default, uses pretty prints)
    Cli,
    /// Error level - least verbose
    Error,
    /// Warn level
    Warn,
    /// Info level
    Info,
    /// Debug level
    Debug,
    /// Trace level - maximum detail
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Cli
    }
}

impl LogLevel {
    /// Converts to tracing-compatible level string.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Cli => "cli",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
    }

    /// Checks if this is the default level.
    pub fn is_default(&self) -> bool {
        matches!(self, LogLevel::Cli)
    }
}
