//! File metadata response for search results.
//! Contains matched file data with AST elements and documentation.

use crate::opensearch::data::requests::opensearch_file_stats_request::OpensearchFileStatsRequest;

/// File metadata returned from OpenSearch search queries.
/// Full file AST and stats, same structure as indexed documents.
/// Alias for OpensearchFileStatsRequest — used in search results.
pub type OpensearchFileMetaResponse = OpensearchFileStatsRequest;
