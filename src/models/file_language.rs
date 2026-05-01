//! Language file model.
//! Represents a parsed source code or Markdown file with all extracted elements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vibe_ast::{
    nodes::{
        class_node::ClassNode, enum_node::EnumNode, function_node::FunctionNode,
        heading_node::HeadingNode, interface_node::InterfaceNode, link_node::LinkNode,
        nodes::Nodes, struct_node::StructNode, variable_node::VariableNode,
    },
    traits::LanguageNode,
};

use crate::models::file_trait::FileModel;

/// Parsed source code or Markdown file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLanguage {
    /// Relative path from project root.
    pub path: String,
    /// Detected programming or markup language.
    pub language: String,
    /// Top-level functions and methods.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<Function>,
    /// Class declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub classes: Vec<Class>,
    /// Struct declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub structs: Vec<Struct>,
    /// Enum declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub enums: Vec<Enum>,
    /// Interface or trait declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub interfaces: Vec<Interface>,
    /// Global variables and constants.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variables: Vec<Variable>,
    /// Import declarations.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,
    /// Module-level documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub header_comments: Vec<String>,
    /// Markdown headings with previews.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub headings: Vec<Heading>,
    /// Markdown links.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub links: Vec<Link>,
    /// Code block languages used in the file.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub code_blocks: Vec<String>,
    /// YAML frontmatter key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub frontmatter: Option<HashMap<String, String>>,
    /// LLM-generated file description.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    /// Search tags (structural + LLM-generated).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl FileModel for FileLanguage {
    fn path(&self) -> &str {
        &self.path
    }
}

impl From<Nodes> for FileLanguage {
    fn from(nodes: Nodes) -> Self {
        match nodes {
            Nodes::Rust(r) => FileLanguage {
                language: r.language_name().into(),
                functions: r.functions.into_iter().map(Into::into).collect(),
                structs: r.structs.into_iter().map(Into::into).collect(),
                enums: r.enums.into_iter().map(Into::into).collect(),
                interfaces: r.interfaces.into_iter().map(Into::into).collect(),
                variables: r.variables.into_iter().map(Into::into).collect(),
                imports: r.imports,
                header_comments: r.header_comments,
                tags: r.tags,
                ..Default::default()
            },
            Nodes::Python(p) => FileLanguage {
                language: p.language_name().into(),
                functions: p.functions.into_iter().map(Into::into).collect(),
                classes: p.classes.into_iter().map(Into::into).collect(),
                variables: p.variables.into_iter().map(Into::into).collect(),
                imports: p.imports,
                header_comments: p.header_comments,
                tags: p.tags,
                ..Default::default()
            },
            Nodes::TypeScript(ts) => FileLanguage {
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
                ..Default::default()
            },
            Nodes::JavaScript(js) => FileLanguage {
                language: js.language_name().into(),
                functions: js.functions.into_iter().map(Into::into).collect(),
                classes: js.classes.into_iter().map(Into::into).collect(),
                variables: js.variables.into_iter().map(Into::into).collect(),
                imports: js.imports,
                header_comments: js.header_comments,
                tags: js.tags,
                ..Default::default()
            },
            Nodes::Java(j) => FileLanguage {
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
                ..Default::default()
            },
            Nodes::Go(g) => FileLanguage {
                language: g.language_name().into(),
                functions: g.functions.into_iter().map(Into::into).collect(),
                structs: g.structs.into_iter().map(Into::into).collect(),
                enums: g.enums.into_iter().map(Into::into).collect(),
                variables: g.variables.into_iter().map(Into::into).collect(),
                imports: g.imports,
                header_comments: g.header_comments,
                tags: g.tags,
                ..Default::default()
            },
            Nodes::CSharp(cs) => FileLanguage {
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
                ..Default::default()
            },
            Nodes::Kotlin(k) => FileLanguage {
                language: k.language_name().into(),
                functions: k.functions.into_iter().map(Into::into).collect(),
                classes: k.classes.into_iter().map(Into::into).collect(),
                enums: k.enums.into_iter().map(Into::into).collect(),
                interfaces: k.interfaces.into_iter().map(Into::into).collect(),
                variables: k.variables.into_iter().map(Into::into).collect(),
                imports: k.imports,
                header_comments: k.header_comments,
                tags: k.tags,
                ..Default::default()
            },
            Nodes::Swift(s) => FileLanguage {
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
                ..Default::default()
            },
            Nodes::Dart(d) => FileLanguage {
                language: d.language_name().into(),
                functions: d.functions.into_iter().map(Into::into).collect(),
                classes: d.classes.into_iter().map(Into::into).collect(),
                enums: d.enums.into_iter().map(Into::into).collect(),
                interfaces: d.interfaces.into_iter().map(Into::into).collect(),
                variables: d.variables.into_iter().map(Into::into).collect(),
                imports: d.imports,
                header_comments: d.header_comments,
                tags: d.tags,
                ..Default::default()
            },
            Nodes::Bash(b) => FileLanguage {
                language: b.language_name().into(),
                functions: b.functions.into_iter().map(Into::into).collect(),
                variables: b.variables.into_iter().map(Into::into).collect(),
                imports: b.imports,
                header_comments: b.header_comments,
                tags: b.tags,
                ..Default::default()
            },
            Nodes::Batch(b) => FileLanguage {
                language: b.language_name().into(),
                functions: b.functions.into_iter().map(Into::into).collect(),
                variables: b.variables.into_iter().map(Into::into).collect(),
                imports: b.imports,
                header_comments: b.header_comments,
                tags: b.tags,
                ..Default::default()
            },
            Nodes::ArkTS(a) => FileLanguage {
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
                ..Default::default()
            },
            Nodes::Markdown(m) => FileLanguage {
                language: m.language_name().into(),
                headings: m.headings.into_iter().map(Into::into).collect(),
                links: m.links.into_iter().map(Into::into).collect(),
                code_blocks: m.code_blocks,
                frontmatter: m.frontmatter,
                tags: m.tags,
                ..Default::default()
            },
        }
    }
}

/// Function or method signature with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Full function signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<FunctionNode> for Function {
    fn from(node: FunctionNode) -> Self {
        Function {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Class declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    /// Full class signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<ClassNode> for Class {
    fn from(node: ClassNode) -> Self {
        Class {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Struct or record declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    /// Full struct signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<StructNode> for Struct {
    fn from(node: StructNode) -> Self {
        Struct {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Enum declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    /// Full enum signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<EnumNode> for Enum {
    fn from(node: EnumNode) -> Self {
        Enum {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Interface or trait declaration with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    /// Full interface signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<InterfaceNode> for Interface {
    fn from(node: InterfaceNode) -> Self {
        Interface {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Global variable or constant with documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    /// Full variable signature.
    pub signature: String,
    /// Documentation comments.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub comments: Vec<String>,
}

impl From<VariableNode> for Variable {
    fn from(node: VariableNode) -> Self {
        Variable {
            signature: node.signature,
            comments: node.comments,
        }
    }
}

/// Markdown heading with level, title, and content preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    /// Heading level (1-6).
    pub level: u8,
    /// Heading text.
    pub title: String,
    /// First non-empty line after the heading.
    pub preview: String,
}

impl From<HeadingNode> for Heading {
    fn from(node: HeadingNode) -> Self {
        Heading {
            level: node.level,
            title: node.title,
            preview: node.preview,
        }
    }
}

/// Markdown link with display text and URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Visible text of the link.
    pub text: String,
    /// Target URL.
    pub url: String,
}

impl From<LinkNode> for Link {
    fn from(node: LinkNode) -> Self {
        Link {
            text: node.text,
            url: node.url,
        }
    }
}
