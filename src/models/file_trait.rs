//! Common file trait with global LRU cache.
//! Provides size and hash accessors with automatic caching.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use lru::LruCache;
use once_cell::sync::Lazy;
use std::num::NonZeroUsize;

/// Cached file metadata.
#[derive(Clone)]
struct CachedMeta {
    /// File size in bytes.
    size_bytes: u64,
    /// BLAKE3 hashes keyed by base_path.
    blake3_hashes: HashMap<PathBuf, String>,
}

/// Global LRU cache for up to 10,000 files.
static FILE_CACHE: Lazy<Mutex<LruCache<PathBuf, CachedMeta>>> =
    Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(10_000).unwrap())));

/// Trait for all file models in the system.
pub trait FileModel {
    /// Relative path from project root.
    fn path(&self) -> &str;

    /// File size in bytes — cached on first access.
    fn size_bytes(&self) -> u64 {
        let path = PathBuf::from(self.path());
        let mut cache = FILE_CACHE.lock().unwrap();
        let entry = cache.get_or_insert(path.clone(), || CachedMeta {
            size_bytes: std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0),
            blake3_hashes: HashMap::new(),
        });
        entry.size_bytes
    }

    /// BLAKE3 hash of file content — cached per base_path.
    fn hash(&self, base_path: &Path) -> String {
        let path = PathBuf::from(self.path());
        let mut cache = FILE_CACHE.lock().unwrap();
        if let Some(entry) = cache.get(&path) {
            if let Some(hash) = entry.blake3_hashes.get(base_path) {
                return hash.clone();
            }
        }
        let full_path = base_path.join(&path);
        let mut hasher = blake3::Hasher::new();
        if let Ok(mut file) = std::fs::File::open(&full_path) {
            let mut buffer = [0; 8192];
            loop {
                match std::io::Read::read(&mut file, &mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        hasher.update(&buffer[..n]);
                    }
                    Err(_) => break,
                }
            }
        }
        let hash = hasher.finalize().to_hex().to_string();
        let mut entry = cache.pop(&path).unwrap_or(CachedMeta {
            size_bytes: 0,
            blake3_hashes: HashMap::new(),
        });
        entry
            .blake3_hashes
            .insert(base_path.to_path_buf(), hash.clone());
        cache.put(path, entry);
        hash
    }
}
