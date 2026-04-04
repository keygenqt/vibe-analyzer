use anyhow::Result;
use blake3::Hasher;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Вычисляет BLAKE3 хэш файла
pub fn from_file(path: &Path) -> Result<String> {
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
