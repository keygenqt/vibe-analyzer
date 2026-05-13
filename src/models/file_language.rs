//! Language file model.
//! Represents a parsed source code or Markdown file with all extracted elements.

use serde::Deserialize;
use serde::Serialize;
use vibe_ast::Language;
use vibe_ast::nodes::nodes::Nodes;
use vibe_ast::traits::LanguageNode;

use crate::analyzer::AnalyzeMode;
use crate::models::file_base::FileBaseModel;
use crate::models::file_language_analyze::FileLanguageAnalyzeModel;
use crate::models::file_language_code::FileLanguageCodeModel;
use crate::models::file_language_knowledge::FileLanguageKnowledgeModel;
use crate::models::trait_file_model::TraitFileModel;
use crate::models::trait_format_model::TraitFormatModel;

/// Parsed source code or Markdown file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguageModel {
    /// Detected programming or markup language.
    pub language: String,
    /// Base file information (path, hash, content, bytes, lines).
    #[serde(flatten)]
    pub base: FileBaseModel,
    /// LLM enrichment data.
    #[serde(flatten)]
    pub analyze: FileLanguageAnalyzeModel,
    /// Code structures extracted from source files.
    #[serde(flatten)]
    pub code: FileLanguageCodeModel,
    /// Knowledge base structures.
    #[serde(flatten)]
    pub knowledge: FileLanguageKnowledgeModel,
    /// Detected programming or markup language.
    #[serde(skip)]
    pub node_language: Option<Language>,
}

impl FileLanguageModel {
    /// Set base file information.
    pub fn set_base(mut self, base: FileBaseModel) -> Self {
        self.base = base;
        self
    }
}

/// File model implementation for language files.
impl TraitFileModel for FileLanguageModel {
    fn base_model(&self) -> &FileBaseModel {
        &self.base
    }
}

/// Format implementation for language files.
impl TraitFormatModel for FileLanguageModel {
    fn format_ast(&self) -> Self {
        FileLanguageModel {
            language: self.language.clone(),
            base: self.base.format_ast(),
            analyze: self.analyze.format_ast(),
            code: self.code.format_ast(),
            knowledge: self.knowledge.format_ast(),
            node_language: None,
        }
    }

    fn format_analyze(&self) -> Self {
        FileLanguageModel {
            language: self.language.clone(),
            base: self.base.format_analyze(),
            analyze: self.analyze.format_analyze(),
            code: self.code.format_analyze(),
            knowledge: self.knowledge.format_analyze(),
            node_language: None,
        }
    }

    fn format_index(&self) -> Self {
        FileLanguageModel {
            language: self.language.clone(),
            base: self.base.format_index(),
            analyze: self.analyze.format_index(),
            code: self.code.format_index(),
            knowledge: self.knowledge.format_index(),
            node_language: None,
        }
    }

    fn format_tool(&self) -> Self {
        FileLanguageModel {
            language: self.language.clone(),
            base: self.base.format_tool(),
            analyze: self.analyze.format_tool(),
            code: self.code.format_tool(),
            knowledge: self.knowledge.format_tool(),
            node_language: None,
        }
    }

    fn format_batch(&self, mode: AnalyzeMode) -> Self {
        Self {
            language: self.language.clone(),
            base: self.base.format_batch(mode),
            analyze: self.analyze.format_batch(mode),
            code: self.code.format_batch(mode),
            knowledge: self.knowledge.format_batch(mode),
            node_language: None,
        }
    }
}

/// Macro to build FileLanguageModel from AST nodes.
macro_rules! from_code_nodes {
    ($nodes:ident, $lang:ident, [$($field:ident),*]) => {
        FileLanguageModel {
            language: $nodes.language_name().into(),
            code: FileLanguageCodeModel {
                // Common fields for all code languages.
                header_comments: $nodes.header_comments,
                imports: $nodes.imports,
                // Language-specific code structures.
                $(
                    $field: $nodes.$field.into_iter().map(Into::into).collect(),
                )*
                // Remaining code fields default to empty.
                ..Default::default()
            },
            analyze: FileLanguageAnalyzeModel {
                // Structural tags from AST parser.
                tags: $nodes.tags,
                // Summary filled later by LLM enrichment.
                ..Default::default()
            },
            node_language: Some(Language::$lang),
            // base, knowledge default to empty.
            ..Default::default()
        }
    };
}

/// Converts parsed AST nodes into a language file model.
#[rustfmt::skip]
impl From<Nodes> for FileLanguageModel {
    fn from(nodes: Nodes) -> Self {
        match nodes {
            // ArkTS: TypeScript-like with full OOP support.
            Nodes::ArkTS(n) => from_code_nodes!(n, ArkTS,
                [functions, classes, structs, enums, interfaces, variables]
            ),
            // Bash: scripting with functions and variables.
            Nodes::Bash(n) => from_code_nodes!(n, Bash,
                [functions, variables]
            ),
            // Batch: Windows scripting with functions and variables.
            Nodes::Batch(n) => from_code_nodes!(n, Batch,
                [functions, variables]
            ),
            // C#: full OOP with all structure types.
            Nodes::CSharp(n) => from_code_nodes!(n, CSharp,
                [functions, classes, structs, enums, interfaces, variables]
            ),
            // Dart: OOP with classes, enums, interfaces, no structs.
            Nodes::Dart(n) => from_code_nodes!(n, Dart,
                [functions, classes, enums, interfaces, variables]
            ),
            // Go: functions, structs, enums, variables, no classes.
            Nodes::Go(n) => from_code_nodes!(n, Go,
                [functions, structs, enums, variables]
            ),
            // Java: full OOP with all structure types.
            Nodes::Java(n) => from_code_nodes!(n, Java,
                [functions, classes, structs, enums, interfaces, variables]
            ),
            // JavaScript: dynamic with functions, classes, variables.
            Nodes::JavaScript(n) => from_code_nodes!(n, JavaScript,
                [functions, classes, variables]
            ),
            // Kotlin: OOP with classes, enums, interfaces, no structs.
            Nodes::Kotlin(n) => from_code_nodes!(n, Kotlin,
                [functions, classes, enums, interfaces, variables]
            ),
            // Python: dynamic with functions, classes, variables.
            Nodes::Python(n) => from_code_nodes!(n, Python,
                [functions, classes, variables]
            ),
            // Rust: functions, structs, enums, traits, variables, no classes.
            Nodes::Rust(n) => from_code_nodes!(n, Rust,
                [functions, structs, enums, interfaces, variables]
            ),
            // Swift: full OOP with all structure types.
            Nodes::Swift(n) => from_code_nodes!(n, Swift,
                [functions, classes, structs, enums, interfaces, variables]
            ),
            // TypeScript: typed JavaScript with full OOP support.
            Nodes::TypeScript(n) => from_code_nodes!(n, TypeScript,
                [functions, classes, structs, enums, interfaces, variables]
            ),
            // Markdown: documentation with headings, links, code blocks, frontmatter.
            Nodes::Markdown(n) => FileLanguageModel {
                language: n.language_name().into(),
                knowledge: FileLanguageKnowledgeModel {
                    headings: n.headings.into_iter().map(Into::into).collect(),
                    links: n.links.into_iter().map(Into::into).collect(),
                    code_blocks: n.code_blocks,
                    frontmatter: n.frontmatter,
                },
                analyze: FileLanguageAnalyzeModel {
                    tags: n.tags,
                    ..Default::default()
                },
                node_language: Some(Language::Markdown),
                ..Default::default()
            },
        }
    }
}
