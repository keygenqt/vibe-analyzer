//! MCP parameter normalization utilities.
//! Fixes common LLM hallucinations in tool parameters.

use std::path::Path;

use rmcp::ErrorData;
use serde_json::json;

use crate::configs::app::IndexType;

/// Normalize target parameter and return list of indexes to search.
/// If target is None, returns all indexes of given type.
pub fn get_targets_normalize(
    config: &crate::configs::app::AppConfig,
    target: &Option<String>,
    index_type: IndexType,
) -> Result<Vec<String>, ErrorData> {
    let target = target_normalize(config, target.as_deref())?;
    match target {
        Some(t) => {
            let index = config.get_index(&t, index_type).map_err(|e| {
                ErrorData::invalid_params(e.to_string(), Some(json!({ "detail": e.to_string() })))
            })?;
            Ok(vec![index])
        }
        None => Ok(config.get_indexes(index_type)),
    }
}

/// Normalize target project parameter.
/// Handles: trim, empty check, hallucinated paths, case-insensitive search.
/// Returns:
/// - `Some(path)` for specific project
/// - `None` for all projects (when target is empty, "null", or missing)
fn target_normalize(
    config: &crate::configs::app::AppConfig,
    target: Option<&str>,
) -> Result<Option<String>, ErrorData> {
    // None, empty, or "null" → search all projects
    let target_str = match target {
        Some(t) if !t.trim().is_empty() && t != "null" => t,
        _ => return Ok(None),
    };

    let target = target_str.trim();

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
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
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
                Ok(Some(matches[0].to_string_lossy().to_string()))
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
