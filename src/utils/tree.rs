//! Directory tree building utilities.
//! Provides functions to generate formatted tree structures from file paths.

use std::collections::{BTreeMap, HashSet};

/// Builds a formatted tree string from a list of file paths.
/// Returns the tree string, number of directories, and number of files within the specified level.
pub fn build_tree(paths: &[String], level: usize) -> (String, usize, usize) {
    let mut dirs = HashSet::new();
    let files = paths
        .iter()
        .filter(|p| p.split('/').count() <= level)
        .count();
    let mut tree: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for path in paths {
        let parts: Vec<&str> = path.split('/').collect();

        for i in 0..parts.len().saturating_sub(1).min(level) {
            dirs.insert(parts[..=i].join("/"));
        }

        for i in 0..parts.len().min(level) {
            let parent = if i == 0 {
                String::new()
            } else {
                parts[..i].join("/")
            };
            let child = parts[i].to_string();
            tree.entry(parent).or_insert_with(Vec::new).push(child);
        }
    }

    for children in tree.values_mut() {
        children.sort();
        children.dedup();
    }

    let tree_string = format_tree(&tree, "", "", level);
    (tree_string, dirs.len(), files)
}

/// Recursively formats the tree structure with proper indentation.
fn format_tree(
    tree: &BTreeMap<String, Vec<String>>,
    prefix: &str,
    current_path: &str,
    max_level: usize,
) -> String {
    let mut result = String::new();

    let children = match tree.get(current_path) {
        Some(c) => c,
        None => return result,
    };

    let depth = if current_path.is_empty() {
        0
    } else {
        current_path.split('/').count()
    };
    if depth >= max_level {
        return result;
    }

    for (i, child) in children.iter().enumerate() {
        let is_last = i == children.len() - 1;
        let branch = if is_last { "`-- " } else { "|-- " };
        result.push_str(&format!("{}{}{}\n", prefix, branch, child));

        let child_path = if current_path.is_empty() {
            child.clone()
        } else {
            format!("{}/{}", current_path, child)
        };
        let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "|   " });
        result.push_str(&format_tree(tree, &next_prefix, &child_path, max_level));
    }

    result
}
