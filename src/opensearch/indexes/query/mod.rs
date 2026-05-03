//! Index-specific query implementations.
//! Each module handles creation or deletion for a specific index type.

pub mod cleanup_orphaned_files;
pub mod cleanup_orphaned_projects;
pub mod create_file_meta;
pub mod create_file_raw;
pub mod create_project_meta;
