//! Multi-project batch export container
//!
//! This module defines [`MultiProjectAnalysis`] for grouping multiple
//! project analysis results into a single JSON export.

use super::project_analysis::ProjectAnalysis;
use serde::{Deserialize, Serialize};

/// Container for multiple project analyses (batch JSON export)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiProjectAnalysis {
    /// ISO 8601 timestamp of the batch export
    pub indexed_at: String,
    /// Collection of individual project analyses
    pub projects: Vec<ProjectAnalysis>,
}
