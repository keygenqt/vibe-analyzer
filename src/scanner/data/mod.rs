//! Data structures for scan results.
//! Defines all serializable types used for source analysis, file metadata, and statistics.

pub mod input {
    pub mod file_info;
    pub mod file_stats;
}

pub mod output {
    pub mod file_analysis;
    pub mod license_info;
    pub mod readme_info;
    pub mod source_analysis;
    pub mod source_stats;
    pub mod sources_analysis;
}
