//! Common file trait.
//! Provides accessors for shared file metadata.

use std::path::Path;

/// Trait for all file models in the system.
#[allow(dead_code)]
pub trait FileModel {
    /// Absolute path to file.
    fn path(&self) -> &str;

    /// Full file content.
    fn content(&self) -> Option<&str> {
        None
    }

    /// File name without extension.
    fn name(&self) -> &str {
        let file_name = self.path().split('/').last().unwrap_or("");
        file_name
            .rsplit_once('.')
            .map(|(name, _)| name)
            .unwrap_or(file_name)
    }

    /// File name with extension.
    fn file_name(&self) -> &str {
        self.path().split('/').last().unwrap_or("")
    }

    /// File extension without dot.
    fn extension(&self) -> Option<String> {
        Path::new(self.path())
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Content preview, breaking at word boundary. Default 100 characters.
    fn preview(&self, size: Option<usize>) -> Option<String> {
        let size = size.unwrap_or(100);
        self.content().and_then(|c| {
            let trimmed = c.trim_start_matches(|ch| ch == ' ' || ch == '\n');
            let chars: Vec<char> = trimmed.chars().take(size).collect();
            if chars.is_empty() {
                return None;
            }
            let preview: String = if let Some(pos) = chars.iter().rposition(|&ch| ch == ' ') {
                chars[..pos].iter().collect()
            } else {
                chars.iter().collect()
            };
            let preview = preview.trim_end();
            if preview.is_empty() {
                None
            } else {
                Some(preview.to_string())
            }
        })
    }

    /// File size in bytes.
    fn size_bytes(&self) -> Option<u64> {
        std::fs::metadata(self.path()).map(|m| m.len()).ok()
    }

    /// BLAKE3 hash of file content.
    fn hash(&self) -> Option<String> {
        use std::io::Read;
        let path = Path::new(self.path());
        let file = std::fs::File::open(path).ok()?;
        let mut reader = std::io::BufReader::new(file);
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(_) => break,
            }
        }
        Some(hasher.finalize().to_hex().to_string())
    }

    /// Number of lines in file.
    fn lines_of_code(&self) -> Option<usize> {
        self.content().map(|c| c.lines().count())
    }

    /// AST-only export: structure without enrichment.
    fn format_ast(&self) -> crate::models::file_models::FileModels;

    /// AST + enrichment export.
    fn format_analyze(&self) -> crate::models::file_models::FileModels;
}
