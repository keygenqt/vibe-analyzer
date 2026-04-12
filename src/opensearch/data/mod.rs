//! OpenSearch document models for indexing.
//! Contains three document types: meta, file, and analysis.

pub mod requests;
pub mod responses;

pub trait HasPath {
    fn path(&self) -> &str;
}
