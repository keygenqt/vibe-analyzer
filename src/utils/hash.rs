//! Cryptographic hashing utilities for file identification.
//! Provides BLAKE3 hash computation for generating unique file IDs.

use anyhow::Result;
use blake3::Hasher;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

/// Computes BLAKE3 hash of file content (cryptographically secure)
pub fn hash_from_content(path: &Path) -> Result<String> {
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

/// Computes BLAKE3 hash from a string path for project identification.
/// Returns first 8 characters of the hash.
pub fn hash_from_path(path: &Path) -> String {
    let mut hasher = Hasher::new();
    hasher.update(path.to_string_lossy().as_bytes());
    hasher.finalize().to_hex()[..8].to_string()
}
