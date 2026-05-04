//! Directory tree and file structure queries.

use anyhow::Result;

use crate::{
    models::file_models::FileModels, opensearch::tools::tool_client::OpenSearchToolClient, utils,
};

/// Directory tree output.
pub struct ShowTreeOutput {
    pub tree: String,
    pub directories: usize,
    pub files: usize,
}

impl OpenSearchToolClient {
    /// Generate directory tree structure from file paths.
    pub async fn show_tree(&self, level: usize) -> Result<ShowTreeOutput> {
        let docs = self.scroll_files_meta().await?;

        let paths: Vec<String> = docs
            .iter()
            .map(|f| match f {
                FileModels::Language(m) => m.path.clone(),
                FileModels::Text(m) => m.path.clone(),
                FileModels::Binary(m) => m.path.clone(),
            })
            .collect();

        if paths.is_empty() {
            return Ok(ShowTreeOutput {
                tree: String::new(),
                directories: 0,
                files: 0,
            });
        }

        let common_root: &str = paths
            .iter()
            .map(|s| s.as_str())
            .reduce(|a, b| {
                let common: String = a
                    .chars()
                    .zip(b.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(c, _)| c)
                    .collect();
                Box::leak(common.into_boxed_str())
            })
            .unwrap_or("");

        let root = common_root
            .rfind('/')
            .map(|i| &common_root[..i])
            .unwrap_or(common_root);

        let (tree, dirs, files) = utils::tree::build_tree(root, &paths, level);

        Ok(ShowTreeOutput {
            tree,
            directories: dirs,
            files,
        })
    }
}
