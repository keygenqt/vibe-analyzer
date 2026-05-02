//! Development and debugging configuration.
//! Contains settings used only during development and testing.

use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::utils::path::get_config_dir;

/// Log level for tracing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Use default level (info)
    Default,
    /// Trace level - maximum detail
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Default
    }
}

impl LogLevel {
    /// Converts to tracing-compatible level string.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Default => "default",
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }

    /// Checks if this is the default level.
    pub fn is_default(&self) -> bool {
        matches!(self, LogLevel::Default)
    }
}

/// Development configuration overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    /// Log level: default, trace, debug, info, warn, error
    #[serde(default)]
    pub log_level: LogLevel,

    /// Path to the directory containing SPDX license data
    #[serde(default)]
    pub spdx_data_path: PathBuf,
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Default,
            spdx_data_path: get_config_dir().join("spdx"),
        }
    }
}

impl DevConfig {
    /// Returns true if dev block should be skipped (i.e., NOT serialized).
    pub fn should_skip(&self) -> bool {
        !self.exists_in_config()
    }

    /// Checks if dev block exists in the config file.
    pub fn exists_in_config(&self) -> bool {
        let config_path = AppConfig::config_path();
        if !config_path.exists() {
            return false;
        }
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            content.contains("log_level")
        } else {
            false
        }
    }

    /// Returns the path to the directory containing license JSON files.
    pub fn spdx_details_path(&self) -> PathBuf {
        self.spdx_data_path.join("json").join("details")
    }

    /// Ensures SPDX data is available, downloading if necessary.
    // pub async fn ensure_spdx_data(&self) -> Result<(), anyhow::Error> {
    //     _spdx::ensure_data(&self.spdx_data_path, self.is_default_path()).await
    // }

    /// Checks if the configured SPDX path is the default one.
    fn is_default_path(&self) -> bool {
        self.spdx_data_path == get_config_dir().join("spdx")
    }
}
