//! JSON5 formatting utilities with comment injection support.
//! Provides types and functions for adding documentation comments to JSON5 configuration files.

/// Comment annotation for a JSON5 field with multiple comment lines.
pub enum Json5 {
    Commit(&'static str, Vec<&'static str>),
}

/// Adds comments before specified fields in a JSON5 string.
pub fn add_commits(json: String, commits: Vec<Json5>) -> String {
    let mut result = json;
    for commit in commits {
        let Json5::Commit(field, comments) = commit;

        let comment_block = comments
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    format!("// {}", c).trim_end().to_string()
                } else {
                    format!("  // {}", c).trim_end().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Support both quoted and unquoted keys
        let pattern_quoted = format!(" \"{}\":", field);
        let pattern_unquoted = format!(" {}:", field);

        let replacement_quoted = format!(" {}\n  \"{}\":", comment_block, field);
        let replacement_unquoted = format!(" {}\n  {}:", comment_block, field);

        if result.contains(&pattern_quoted) {
            result = result.replace(&pattern_quoted, &replacement_quoted);
        } else if result.contains(&pattern_unquoted) {
            result = result.replace(&pattern_unquoted, &replacement_unquoted);
        }
    }
    result
}
