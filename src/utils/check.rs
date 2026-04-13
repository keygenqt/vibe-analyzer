//! MIME type detection utilities for file content classification.
//! Uses multiple strategies (extension, filename, content sniffing) to determine MIME types.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Detects if a file is text using extension, filename, and content analysis.
pub fn is_text(path: &Path) -> bool {
    // 1. Check by file extension (fast path)
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        const TEXT_EXTS: &[&str] = &[
            "rs",
            "py",
            "go",
            "java",
            "c",
            "cpp",
            "h",
            "hpp",
            "cs",
            "swift",
            "dart",
            "rb",
            "php",
            "hs",
            "kt",
            "kts",
            "js",
            "jsx",
            "ts",
            "tsx",
            "ets",
            "arkts",
            "bat",
            "cmd",
            "sh",
            "bash",
            "zsh",
            "json",
            "toml",
            "yaml",
            "yml",
            "xml",
            "ini",
            "cfg",
            "conf",
            "properties",
            "lock",
            "env",
            "md",
            "markdown",
            "txt",
            "log",
            "rst",
            "adoc",
            "html",
            "htm",
            "css",
            "scss",
            "sass",
            "less",
            "svg",
        ];

        if TEXT_EXTS.contains(&ext.to_lowercase().as_str()) {
            return true;
        }
    }

    // 2. Check by exact filename (no extension)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        const TEXT_NAMES: &[&str] = &[
            "Dockerfile",
            "Makefile",
            "Cargo.lock",
            ".gitignore",
            ".gitattributes",
            ".env",
            "LICENSE",
            "README",
            "CHANGELOG",
        ];

        if TEXT_NAMES.contains(&name) {
            return true;
        }
    }

    // 3. Analyze file content (reliable fallback)
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 256];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read > 0 {
                let content = &buffer[..bytes_read];
                // Text if no null bytes and >50% printable ASCII characters
                if !content.contains(&0) {
                    let printable = content
                        .iter()
                        .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
                        .count();
                    return printable > content.len() / 2;
                }
            }
        }
    }

    false
}
