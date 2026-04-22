//! Data structures for scan results.
//! Defines all serializable types used for source analysis, file metadata, and statistics.

pub mod input {
    pub mod scan_file_info_input;
    pub mod scan_file_stats_input;
}

pub mod output {
    pub mod scan_file_analysis_output;
    pub mod scan_license_output;
    pub mod scan_project_analysis_output;
    pub mod scan_project_stats_output;
    pub mod scan_projects_analysis_output;
    pub mod scan_readme_output;
}
