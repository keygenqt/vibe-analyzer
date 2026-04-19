//! MCP parameter normalization utilities.
//! Fixes common LLM hallucinations in tool parameters.

use std::path::Path;

use rmcp::ErrorData;
use serde_json::json;

/// Normalize target project parameter.
/// Handles: trim, empty check, hallucinated paths, case-insensitive search.
/// If target is None and only one project exists, returns it.
pub fn target_normalize(
    config: &crate::configs::app::AppConfig,
    target: Option<&str>,
) -> Result<String, ErrorData> {
    let target = match target {
        Some(t) => t,
        None => {
            let sources = config.get_sources();
            if sources.len() == 1 {
                return Ok(sources[0].to_string_lossy().to_string());
            } else {
                return Err(ErrorData::invalid_params(
                    format!(
                        "No target specified and {} projects available. Use list_projects to choose.",
                        sources.len()
                    ),
                    Some(json!({ "hint": "call list_projects", "available": sources.len() })),
                ));
            }
        }
    };

    let target = target.trim();

    if target.is_empty() {
        return Err(ErrorData::invalid_params(
            "Target is empty. Use list_projects to see available projects.",
            Some(json!({ "hint": "call list_projects" })),
        ));
    }

    // If model hallucinated a path like "/path/to/knowledge", extract the last part
    let target = if target.starts_with('/') || target.starts_with("./") || target.starts_with("../")
    {
        Path::new(target)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(target)
    } else {
        target
    };

    // Try exact match first
    match config.get_source(target) {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(_) => {
            // Try case-insensitive match
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
                Ok(matches[0].to_string_lossy().to_string())
            } else if matches.len() > 1 {
                Err(ErrorData::invalid_params(
                    format!("Multiple projects match '{}'. Use full path.", target),
                    Some(
                        json!({ "matches": matches.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>() }),
                    ),
                ))
            } else {
                Err(ErrorData::invalid_params(
                    format!(
                        "Project '{}' not found. Use list_projects to see available projects.",
                        target
                    ),
                    Some(json!({ "hint": "call list_projects" })),
                ))
            }
        }
    }
}

/// Normalize numeric parameter to safe range.
/// - value <= min → default
/// - value > max → max
pub fn number_normalize(value: i32, min: i32, max: i32, default: i32) -> usize {
    if value <= min {
        default as usize
    } else if value > max {
        max as usize
    } else {
        value as usize
    }
}
