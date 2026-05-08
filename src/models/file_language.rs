//! Language file model.
//! Represents a parsed source code or Markdown file with all extracted elements.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use vibe_ast::Language;
use vibe_ast::nodes::nodes::Nodes;
use vibe_ast::traits::LanguageNode;

use crate::models::ast_class::AstClassModel;
use crate::models::ast_enum::AstEnumModel;
use crate::models::ast_function::AstFunctionModel;
use crate::models::ast_heading::AstHeadingModel;
use crate::models::ast_interface::AstInterfaceModel;
use crate::models::ast_link::AstLinkModel;
use crate::models::ast_model::AstModel;
use crate::models::ast_struct::AstStructModel;
use crate::models::ast_variable::AstVariableModel;
use crate::models::file_model::FileModel;
use crate::models::file_models::FileModels;

/// Parsed source code or Markdown file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageModel {
    /// Absolute path to file.
    pub path: String,
    /// Detected programming or markup language.
    pub language: String,
    /// LLM-generated file summary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub summary: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size_bytes: Option<u64>,
    /// Number of lines.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lines_of_code: Option<usize>,
    /// Top-level functions and methods.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<AstFunctionModel>,
    /// Class declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub classes: Vec<AstClassModel>,
    /// Struct declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub structs: Vec<AstStructModel>,
    /// Enum declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub enums: Vec<AstEnumModel>,
    /// Interface or trait declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub interfaces: Vec<AstInterfaceModel>,
    /// Global variables and constants.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variables: Vec<AstVariableModel>,
    /// Import declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub header_comments: Vec<String>,
    /// Markdown headings with previews.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub headings: Vec<AstHeadingModel>,
    /// Markdown links.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub links: Vec<AstLinkModel>,
    /// Code block languages used in the file.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub code_blocks: Vec<String>,
    /// YAML frontmatter key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub frontmatter: Option<HashMap<String, String>>,
    /// Search tags (structural + LLM-generated).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
    /// BLAKE3 hash.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hash: Option<String>,
    /// Full file content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    /// Detected programming or markup language.
    #[serde(skip)]
    pub node_language: Option<Language>,
}

impl FileLanguageModel {
    /// Set absolute file path.
    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_string_lossy().to_string();
        self
    }

    /// Set full file content.
    pub fn with_content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }
}

/// File model implementation for language files.
impl FileModel for FileLanguageModel {
    fn path(&self) -> &str {
        &self.path
    }

    fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    fn format_ast(&self) -> FileModels {
        FileModels::Language(FileLanguageModel {
            path: self.path.clone(),
            language: self.language.clone(),
            summary: None,
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            lines_of_code: self.lines_of_code.or_else(|| self.lines_of_code()),
            functions: self.functions.iter().map(|f| f.format_ast()).collect(),
            classes: self.classes.iter().map(|c| c.format_ast()).collect(),
            structs: self.structs.iter().map(|s| s.format_ast()).collect(),
            enums: self.enums.iter().map(|e| e.format_ast()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_ast()).collect(),
            variables: self.variables.iter().map(|v| v.format_ast()).collect(),
            imports: self.imports.clone(),
            header_comments: self.header_comments.clone(),
            headings: self.headings.iter().map(|h| h.format_ast()).collect(),
            links: self.links.iter().map(|l| l.format_ast()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
            tags: Vec::new(),
            hash: None,
            content: None,
            node_language: None,
        })
    }

    fn format_analyze(&self) -> FileModels {
        FileModels::Language(FileLanguageModel {
            path: self.path.clone(),
            language: self.language.clone(),
            summary: self.summary.clone(),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            lines_of_code: self.lines_of_code.or_else(|| self.lines_of_code()),
            functions: self.functions.iter().map(|f| f.format_analyze()).collect(),
            classes: self.classes.iter().map(|c| c.format_analyze()).collect(),
            structs: self.structs.iter().map(|s| s.format_analyze()).collect(),
            enums: self.enums.iter().map(|e| e.format_analyze()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_analyze()).collect(),
            variables: self.variables.iter().map(|v| v.format_analyze()).collect(),
            imports: self.imports.clone(),
            header_comments: self.header_comments.clone(),
            headings: self.headings.iter().map(|h| h.format_analyze()).collect(),
            links: self.links.iter().map(|l| l.format_analyze()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
            tags: self.tags.clone(),
            hash: None,
            content: None,
            node_language: None,
        })
    }

    fn format_index(&self) -> FileModels {
        FileModels::Language(FileLanguageModel {
            path: self.path.clone(),
            language: self.language.clone(),
            summary: self.summary.clone(),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            lines_of_code: self.lines_of_code.or_else(|| self.lines_of_code()),
            functions: self.functions.iter().map(|f| f.format_index()).collect(),
            classes: self.classes.iter().map(|c| c.format_index()).collect(),
            structs: self.structs.iter().map(|s| s.format_index()).collect(),
            enums: self.enums.iter().map(|e| e.format_index()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_index()).collect(),
            variables: self.variables.iter().map(|v| v.format_index()).collect(),
            imports: self.imports.clone(),
            header_comments: self.header_comments.clone(),
            headings: self.headings.iter().map(|h| h.format_index()).collect(),
            links: self.links.iter().map(|l| l.format_index()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
            tags: self.tags.clone(),
            hash: None,
            content: None,
            node_language: None,
        })
    }

    fn format_tool(&self) -> FileModels {
        FileModels::Language(FileLanguageModel {
            path: self.path.clone(),
            language: self.language.clone(),
            size_bytes: self.size_bytes.or_else(|| self.size_bytes()),
            functions: self.functions.iter().map(|f| f.format_tool()).collect(),
            classes: self.classes.iter().map(|c| c.format_tool()).collect(),
            structs: self.structs.iter().map(|s| s.format_tool()).collect(),
            enums: self.enums.iter().map(|e| e.format_tool()).collect(),
            interfaces: self.interfaces.iter().map(|i| i.format_tool()).collect(),
            variables: self.variables.iter().map(|v| v.format_tool()).collect(),
            imports: self.imports.clone(),
            header_comments: self.header_comments.clone(),
            headings: self.headings.iter().map(|h| h.format_tool()).collect(),
            links: self.links.iter().map(|l| l.format_tool()).collect(),
            code_blocks: self.code_blocks.clone(),
            frontmatter: self.frontmatter.clone(),
            tags: vec![],
            summary: None,
            lines_of_code: None,
            hash: None,
            content: None,
            node_language: None,
        })
    }
}

/// Converts parsed AST nodes into a language file model.
impl From<Nodes> for FileLanguageModel {
    fn from(nodes: Nodes) -> Self {
        match nodes {
            Nodes::ArkTS(a) => FileLanguageModel {
                language: a.language_name().into(),
                functions: a.functions.into_iter().map(Into::into).collect(),
                classes: a.classes.into_iter().map(Into::into).collect(),
                structs: a.structs.into_iter().map(Into::into).collect(),
                enums: a.enums.into_iter().map(Into::into).collect(),
                interfaces: a.interfaces.into_iter().map(Into::into).collect(),
                variables: a.variables.into_iter().map(Into::into).collect(),
                imports: a.imports,
                header_comments: a.header_comments,
                tags: a.tags,
                node_language: Some(Language::ArkTS),
                ..Default::default()
            },
            Nodes::Bash(b) => FileLanguageModel {
                language: b.language_name().into(),
                functions: b.functions.into_iter().map(Into::into).collect(),
                variables: b.variables.into_iter().map(Into::into).collect(),
                imports: b.imports,
                header_comments: b.header_comments,
                tags: b.tags,
                node_language: Some(Language::Bash),
                ..Default::default()
            },
            Nodes::Batch(b) => FileLanguageModel {
                language: b.language_name().into(),
                functions: b.functions.into_iter().map(Into::into).collect(),
                variables: b.variables.into_iter().map(Into::into).collect(),
                imports: b.imports,
                header_comments: b.header_comments,
                tags: b.tags,
                node_language: Some(Language::Batch),
                ..Default::default()
            },
            Nodes::CSharp(cs) => FileLanguageModel {
                language: cs.language_name().into(),
                functions: cs.functions.into_iter().map(Into::into).collect(),
                classes: cs.classes.into_iter().map(Into::into).collect(),
                structs: cs.structs.into_iter().map(Into::into).collect(),
                enums: cs.enums.into_iter().map(Into::into).collect(),
                interfaces: cs.interfaces.into_iter().map(Into::into).collect(),
                variables: cs.variables.into_iter().map(Into::into).collect(),
                imports: cs.imports,
                header_comments: cs.header_comments,
                tags: cs.tags,
                node_language: Some(Language::CSharp),
                ..Default::default()
            },
            Nodes::Dart(d) => FileLanguageModel {
                language: d.language_name().into(),
                functions: d.functions.into_iter().map(Into::into).collect(),
                classes: d.classes.into_iter().map(Into::into).collect(),
                enums: d.enums.into_iter().map(Into::into).collect(),
                interfaces: d.interfaces.into_iter().map(Into::into).collect(),
                variables: d.variables.into_iter().map(Into::into).collect(),
                imports: d.imports,
                header_comments: d.header_comments,
                tags: d.tags,
                node_language: Some(Language::Dart),
                ..Default::default()
            },
            Nodes::Go(g) => FileLanguageModel {
                language: g.language_name().into(),
                functions: g.functions.into_iter().map(Into::into).collect(),
                structs: g.structs.into_iter().map(Into::into).collect(),
                enums: g.enums.into_iter().map(Into::into).collect(),
                variables: g.variables.into_iter().map(Into::into).collect(),
                imports: g.imports,
                header_comments: g.header_comments,
                tags: g.tags,
                node_language: Some(Language::Go),
                ..Default::default()
            },
            Nodes::Java(j) => FileLanguageModel {
                language: j.language_name().into(),
                functions: j.functions.into_iter().map(Into::into).collect(),
                classes: j.classes.into_iter().map(Into::into).collect(),
                structs: j.structs.into_iter().map(Into::into).collect(),
                enums: j.enums.into_iter().map(Into::into).collect(),
                interfaces: j.interfaces.into_iter().map(Into::into).collect(),
                variables: j.variables.into_iter().map(Into::into).collect(),
                imports: j.imports,
                header_comments: j.header_comments,
                tags: j.tags,
                node_language: Some(Language::Java),
                ..Default::default()
            },
            Nodes::JavaScript(js) => FileLanguageModel {
                language: js.language_name().into(),
                functions: js.functions.into_iter().map(Into::into).collect(),
                classes: js.classes.into_iter().map(Into::into).collect(),
                variables: js.variables.into_iter().map(Into::into).collect(),
                imports: js.imports,
                header_comments: js.header_comments,
                tags: js.tags,
                node_language: Some(Language::JavaScript),
                ..Default::default()
            },
            Nodes::Kotlin(k) => FileLanguageModel {
                language: k.language_name().into(),
                functions: k.functions.into_iter().map(Into::into).collect(),
                classes: k.classes.into_iter().map(Into::into).collect(),
                enums: k.enums.into_iter().map(Into::into).collect(),
                interfaces: k.interfaces.into_iter().map(Into::into).collect(),
                variables: k.variables.into_iter().map(Into::into).collect(),
                imports: k.imports,
                header_comments: k.header_comments,
                tags: k.tags,
                node_language: Some(Language::Kotlin),
                ..Default::default()
            },
            Nodes::Markdown(m) => FileLanguageModel {
                language: m.language_name().into(),
                headings: m.headings.into_iter().map(Into::into).collect(),
                links: m.links.into_iter().map(Into::into).collect(),
                code_blocks: m.code_blocks,
                frontmatter: m.frontmatter,
                tags: m.tags,
                node_language: Some(Language::Markdown),
                ..Default::default()
            },
            Nodes::Python(p) => FileLanguageModel {
                language: p.language_name().into(),
                functions: p.functions.into_iter().map(Into::into).collect(),
                classes: p.classes.into_iter().map(Into::into).collect(),
                variables: p.variables.into_iter().map(Into::into).collect(),
                imports: p.imports,
                header_comments: p.header_comments,
                tags: p.tags,
                node_language: Some(Language::Python),
                ..Default::default()
            },
            Nodes::Rust(r) => FileLanguageModel {
                language: r.language_name().into(),
                functions: r.functions.into_iter().map(Into::into).collect(),
                structs: r.structs.into_iter().map(Into::into).collect(),
                enums: r.enums.into_iter().map(Into::into).collect(),
                interfaces: r.interfaces.into_iter().map(Into::into).collect(),
                variables: r.variables.into_iter().map(Into::into).collect(),
                imports: r.imports,
                header_comments: r.header_comments,
                tags: r.tags,
                node_language: Some(Language::Rust),
                ..Default::default()
            },
            Nodes::Swift(s) => FileLanguageModel {
                language: s.language_name().into(),
                functions: s.functions.into_iter().map(Into::into).collect(),
                classes: s.classes.into_iter().map(Into::into).collect(),
                structs: s.structs.into_iter().map(Into::into).collect(),
                enums: s.enums.into_iter().map(Into::into).collect(),
                interfaces: s.interfaces.into_iter().map(Into::into).collect(),
                variables: s.variables.into_iter().map(Into::into).collect(),
                imports: s.imports,
                header_comments: s.header_comments,
                tags: s.tags,
                node_language: Some(Language::Swift),
                ..Default::default()
            },
            Nodes::TypeScript(ts) => FileLanguageModel {
                language: ts.language_name().into(),
                functions: ts.functions.into_iter().map(Into::into).collect(),
                classes: ts.classes.into_iter().map(Into::into).collect(),
                structs: ts.structs.into_iter().map(Into::into).collect(),
                enums: ts.enums.into_iter().map(Into::into).collect(),
                interfaces: ts.interfaces.into_iter().map(Into::into).collect(),
                variables: ts.variables.into_iter().map(Into::into).collect(),
                imports: ts.imports,
                header_comments: ts.header_comments,
                tags: ts.tags,
                node_language: Some(Language::TypeScript),
                ..Default::default()
            },
        }
    }
}
