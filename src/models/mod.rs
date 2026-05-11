//! File models for the vibe-analyzer pipeline.
//! Three file types with shared trait for common accessors.

pub mod file_base;
pub mod file_binary;
pub mod file_language;
pub mod file_language_analyze;
pub mod file_language_code;
pub mod file_language_knowledge;
pub mod file_models;
pub mod file_text;
pub mod lang_class;
pub mod lang_enum;
pub mod lang_function;
pub mod lang_heading;
pub mod lang_interface;
pub mod lang_link;
pub mod lang_struct;
pub mod lang_variable;
pub mod project;
pub mod trait_base_model;
pub mod trait_file_model;
pub mod trait_format_model;
