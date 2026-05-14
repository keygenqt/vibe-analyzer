//! MCP parameter normalization utilities.
//! Fixes common LLM hallucinations in tool parameters.

use anyhow::Result;
use std::path::Path;

use crate::configs::app::AppConfig;
use crate::utils::constants::MAX_LLM_BODY_CHARS;
use crate::utils::constants::MCP_MAX_SEARCH_LIMIT;

/// Splits tags on boundaries and CamelCase with Unicode support, removing duplicates and short words.
pub fn tags_normalize(tags: Vec<String>) -> Vec<String> {
    let mut unique_tags = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut prepared = String::new();
    let mut chars = Vec::new();
    for tag in tags {
        prepared.clear();
        chars.clear();
        chars.extend(tag.chars());
        for i in 0..chars.len() {
            let c = chars[i];
            if !c.is_alphanumeric() {
                prepared.push(' ');
                continue;
            }
            if i > 0 && chars[i - 1].is_alphanumeric() {
                let prev_upper = chars[i - 1].is_uppercase();
                let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
                if (c.is_uppercase() && !prev_upper)
                    || (c.is_uppercase() && prev_upper && next_lower)
                {
                    prepared.push(' ');
                }
            }
            prepared.push(c);
        }
        for word in prepared.split_whitespace() {
            let normalized = word.to_lowercase();
            if normalized.chars().count() >= 3 && unique_tags.insert(normalized.clone()) {
                result.push(normalized);
            }
        }
    }
    result
}

/// Truncate body to max chars for LLM enrichment.
pub fn body_batch_normalize(body: &Option<String>) -> Option<String> {
    body.as_ref().map(|text| {
        if text.len() > MAX_LLM_BODY_CHARS {
            let mut end = MAX_LLM_BODY_CHARS;
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            format!("{} [truncated]", &text[..end])
        } else {
            text.clone()
        }
    })
}

/// Normalizes a file path.
pub fn path_normalize(config: &AppConfig, path: &str) -> Result<String> {
    let path = path.trim();

    // Path cannot be empty, wildcard, or null string
    if path.is_empty() || path == "*" || path == "null" {
        anyhow::bail!("Invalid path: {}", path);
    }

    // Already an absolute path in a known project
    let sources = config.get_sources();
    for source in &sources {
        let root = source.to_string_lossy().to_string();
        if path.starts_with(&root) {
            return Ok(path.to_string());
        }
    }

    // Absolute path outside known projects — extract filename
    if path.starts_with('/') {
        if let Some(filename) = std::path::Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
        {
            return Ok(filename.to_string());
        }
        anyhow::bail!("Cannot extract filename from path: {}", path);
    }

    // Relative path
    Ok(path.to_string())
}

/// Normalize search query.
/// - Replaces wildcards (*, ?) with spaces
/// - Collapses multiple spaces
/// - Trims whitespace
/// - Returns None if query matches all (empty or only wildcards)
pub fn query_normalize(query: &str) -> Option<String> {
    let cleaned = query
        .replace('*', " ")
        .replace('?', " ")
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// Normalize limit parameter to safe range.
/// - None → default
/// - Some(v) where v <= 1 → default
/// - Some(v) where v > max → max
pub fn number_normalize(value: Option<i32>, default: Option<i32>, max: Option<i32>) -> usize {
    let default = default.unwrap_or(10) as usize;
    let max = max.unwrap_or(MCP_MAX_SEARCH_LIMIT) as usize;
    match value {
        None => default,
        Some(v) if v <= 1 => default,
        Some(v) if v > max as i32 => max,
        Some(v) => v as usize,
    }
}

/// Normalize target project parameter.
pub fn target_normalize(
    config: &crate::configs::app::AppConfig,
    target: Option<&str>,
) -> Result<Option<String>> {
    let target_str = match target {
        Some(t) if !t.trim().is_empty() && t != "null" => t,
        _ => return Ok(None),
    };

    let target = target_str.trim();

    let target = if target.starts_with('/') || target.starts_with("./") || target.starts_with("../")
    {
        Path::new(target)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(target)
    } else {
        target
    };

    match config.get_source(target) {
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
        Err(_) => {
            let target_lower = target.to_lowercase();
            let sources = config.get_sources();

            let matches: Vec<_> = sources
                .iter()
                .filter(|s| {
                    s.to_string_lossy().to_lowercase().contains(&target_lower)
                        || s.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.to_lowercase() == target_lower)
                            .unwrap_or(false)
                })
                .collect();

            if matches.len() == 1 {
                Ok(Some(matches[0].to_string_lossy().to_string()))
            } else if matches.len() > 1 {
                anyhow::bail!(
                    "Multiple projects match '{}'. Use full path.\nMatches: {:?}",
                    target,
                    matches
                        .iter()
                        .map(|p| p.to_string_lossy())
                        .collect::<Vec<_>>()
                )
            } else {
                anyhow::bail!(
                    "Project '{}' not found. Use show_projects to see available projects.",
                    target
                )
            }
        }
    }
}
