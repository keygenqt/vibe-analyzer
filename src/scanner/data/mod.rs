//! Data structures for scan results.
//! Defines all serializable types used for project analysis, file metadata, and statistics.

pub mod input {
    pub mod file_info;
}

pub mod output {
    pub mod file_analysis;
    pub mod license_info;
    pub mod project_analysis;
    pub mod project_stats;
    pub mod projects_analysis;
    pub mod readme_info;
}
