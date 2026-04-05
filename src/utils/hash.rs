//! Cryptographic hashing utilities
//!
//! This module provides BLAKE3 hashing for generating unique
//! file identifiers (file_id) used for content linking.

use anyhow::Result;
use blake3::Hasher;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Computes BLAKE3 hash of file content (cryptographically secure)
///
/// # Arguments
/// * `path` - Path to the file to hash
///
/// # Returns
/// * `Result<String>` - Hex-encoded BLAKE3 hash string on success
pub fn file_id_from_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}
