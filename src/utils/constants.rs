//! Application constants and configuration defaults.
//! Defines ignore patterns, file size limits, and configuration file names.

/// Config directory name
pub const CONFIG_DIR_NAME: &str = ".vibe-analyzer";

/// Config file name
pub const CONFIG_FILE_NAME: &str = "config.json5";

/// Config version
pub const CONFIG_VERSION: &str = "0.0.3";

/// Maximum file size for AST parsing (10 MB)
pub const MAX_AST_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum body size sent to LLM enrichment (characters).
pub const MAX_LLM_BODY_CHARS: usize = 2048;

/// Maximum limit for search results in MCP tools.
pub const MCP_MAX_SEARCH_LIMIT: i32 = 10;
