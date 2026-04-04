use anyhow::Result;
use mime::Mime;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Detects MIME type of a file using multiple strategies
pub fn detect_mime(path: &Path) -> Result<Mime> {
    // Strategy 1: Check by file extension
    if let Some(mime) = detect_by_extension(path) {
        return Ok(mime);
    }

    // Strategy 2: Check by exact filename
    if let Some(mime) = detect_by_filename(path) {
        return Ok(mime);
    }

    // Strategy 3: Use mime_guess crate
    if let Some(mime) = detect_by_mime_guess(path) {
        return Ok(mime);
    }

    // Strategy 4: Peek content to detect text vs binary
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 256];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            let content = &buffer[..bytes_read];
            // Text if no null bytes and >50% printable chars
            if !content.contains(&0) {
                let printable = content
                    .iter()
                    .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
                    .count();
                if printable > content.len() / 2 {
                    return Ok(mime::TEXT_PLAIN);
                }
            }
        }
    }

    Ok(mime::APPLICATION_OCTET_STREAM)
}

/// Detects MIME by file extension (custom mappings)
fn detect_by_extension(path: &Path) -> Option<Mime> {
    let ext = path.extension()?.to_str()?;
    let mime = match ext {
        "ui" | "xml" | "qrc" | "xcstrings" | "entitlements" | "appxmanifest" | "resw" | "resx"
        | "csproj" | "slnx" | "xaml" => mime::TEXT_XML,
        "qml" | "po" | "pot" | "spec" | "desktop" | "pro" | "properties" | "lock" | "env"
        | "example" | "in" | "pbxproj" | "sln" | "build" | "wrap" => mime::TEXT_PLAIN,
        _ => return None,
    };
    Some(mime)
}

/// Detects MIME by exact filename (no extension)
fn detect_by_filename(path: &Path) -> Option<Mime> {
    let name = path.file_name()?.to_str()?;
    match name {
        "Dockerfile" | "Makefile" | "Cargo.lock" | ".gitignore" | ".gitattributes" | ".env"
        | ".env.example" => Some(mime::TEXT_PLAIN),
        _ => None,
    }
}

/// Detects MIME using mime_guess crate (standard extension mappings)
fn detect_by_mime_guess(path: &Path) -> Option<Mime> {
    let ext = path.extension()?.to_str()?;
    mime_guess::from_ext(ext).first()
}

/// Checks if a file should be treated as text based on MIME type or extension
pub fn is_text(mime: &Mime, path: &Path) -> bool {
    // Check by MIME type
    if mime.type_() == mime::TEXT {
        return true;
    }

    let subtype = mime.subtype().as_str();
    if matches!(
        subtype,
        "json" | "javascript" | "xml" | "x-sql" | "x-sh" | "toml" | "yaml"
    ) {
        return true;
    }

    // Fallback: check by extension
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(
            ext,
            "rs" | "kt"
                | "kts"
                | "py"
                | "go"
                | "js"
                | "ts"
                | "cs"
                | "swift"
                | "rb"
                | "php"
                | "dart"
                | "hs"
                | "arkts"
                | "ets"
                | "sql"
        )
    } else {
        false
    }
}
