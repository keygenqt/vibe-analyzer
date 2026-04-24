//! OpenSearch document models for indexing and search results.
//! Request models for bulk indexing, response models for search queries.

pub mod requests;
pub mod responses;

/// Common trait for documents that have a file path.
pub trait HasPath {
    fn path(&self) -> &str;
}
