//! OpenSearch document models for indexing.
//! Contains three document types: meta, file, and analysis.

pub mod source_file;
pub mod source_file_analysis;
pub mod source_meta;

pub trait HasPath {
    fn path(&self) -> &str;
}
