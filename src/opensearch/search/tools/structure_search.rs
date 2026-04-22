//! Search functionality for OpenSearch indices.
//! Provides meta, ast, file, and get queries.

use crate::opensearch::data::responses::opensearch_structure_response::OpensearchStructureResponse;
use crate::opensearch::search::search_client::SearchClient;
use anyhow::Result;

impl SearchClient {
    /// Generate directory tree structure from file paths.
    pub async fn structure(
        &self,
        index: &str,
        level: usize,
    ) -> Result<OpensearchStructureResponse> {
        if !self.client.index_exists(index).await? {
            return Ok(OpensearchStructureResponse {
                tree: String::new(),
                directories: 0,
                files: 0,
            });
        }

        let hashes = self.get_hashes(index).await?;
        if hashes.is_empty() {
            return Ok(OpensearchStructureResponse {
                tree: String::new(),
                directories: 0,
                files: 0,
            });
        }

        // Find common root prefix of all paths
        let common_root = hashes
            .iter()
            .map(|h| h.path.as_str())
            .reduce(|a, b| {
                let common: String = a
                    .chars()
                    .zip(b.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(c, _)| c)
                    .collect();
                Box::leak(Box::new(common)) as &str
            })
            .unwrap_or("");

        // Trim to last '/' to get the project root directory
        let root = if let Some(last_slash) = common_root.rfind('/') {
            &common_root[..last_slash]
        } else {
            common_root
        };
        // Collect all paths
        let paths: Vec<String> = hashes.iter().map(|h| h.path.clone()).collect();
        // Build tree
        let (tree, dirs, files) = crate::utils::tree::build_tree(root, &paths, level);
        // Result
        Ok(OpensearchStructureResponse {
            tree,
            directories: dirs,
            files,
        })
    }
}
