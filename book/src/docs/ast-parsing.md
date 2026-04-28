# AST Parsing

Vibe Analyzer uses **tree-sitter** — an incremental parser that builds a concrete syntax tree (CST) from source code. The CST is then traversed to extract meaningful elements: functions, classes, imports, variables, documentation, and multilingual search tags.

## How It Works

```
Source code → tree-sitter → CST → recursive traversal → AstData (structured data + tags)
```

Process for each file:

1. **Parser selection** — the appropriate `LanguageParser` is chosen from the static `PARSERS` registry based on file extension
2. **CST parsing** — tree-sitter builds the tree
3. **Recursive traversal** — the `visit_node` function traverses all nodes and collects meaningful elements
4. **Post-processing** — deduplication, sorting, trimming function/class bodies
5. **Tag generation** — `with_tags()` adds multilingual tags (EN/RU/ZH)

## Parser Registry

| Extensions          | Parser             |
| ------------------- | ------------------ |
| `rs`                | `RustParser`       |
| `py`                | `PythonParser`     |
| `js`, `jsx`         | `JavaScriptParser` |
| `ts`, `tsx`         | `TypeScriptParser` |
| `java`              | `JavaParser`       |
| `go`                | `GoParser`         |
| `cs`                | `CSharpParser`     |
| `kt`, `kts`         | `KotlinParser`     |
| `swift`             | `SwiftParser`      |
| `dart`              | `DartParser`       |
| `sh`, `bash`, `zsh` | `BashParser`       |
| `bat`, `cmd`        | `BatchParser`      |
| `ets`, `arkts`      | `ArkTsParser`      |
| `md`, `markdown`    | `MarkdownParser`   |

Each parser implements the `LanguageParser` trait:

```rust
pub trait LanguageParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<AstData>;
    fn language_name(&self) -> &'static str;
}
```

## AstData Structure

```rust
pub struct AstData {
    pub header_comments: Vec<String>,                 // Module comments
    pub imports: Vec<String>,                         // Imports
    pub variables: Vec<AstDataVariable>,              // Variables/constants
    pub functions: Vec<AstDataFunction>,              // Functions/methods
    pub classes: Vec<AstDataClass>,                   // Classes
    pub structs: Vec<AstDataStruct>,                  // Structs
    pub enums: Vec<AstDataEnum>,                      // Enums
    pub interfaces: Vec<AstDataInterface>,            // Interfaces/traits
    pub frontmatter: Option<HashMap<String, String>>, // Frontmatter (Markdown)
    pub headings: Vec<AstHeading>,                    // Headings (Markdown)
    pub links: Vec<AstLink>,                          // Links (Markdown)
    pub code_blocks: Vec<String>,                     // Code block languages (Markdown)
    pub tags: Vec<String>,                            // Multilingual tags
}
```

Each element (function, class, etc.) has a signature and doc comments:

```rust
pub struct AstDataFunction {
    pub signature: String,     // "pub async fn scan_source(source_path: &Path, ...)"
    pub comments: Vec<String>, // ["Scans a source and returns complete analysis results"]
}
```

## Comment Extraction

Vibe Analyzer distinguishes three types of comments:

### 1. Header Comments

Describe the purpose of the entire file. Stored in `header_comments`.

**Detection rules:**

- The comment is at the **beginning of the file** (first node in the CST)
- Or all preceding sibling nodes are also comments (`is_module_comment`)
- Not inside a function or class

**Syntax by language:**

| Language    | Syntax                             | Parser                                                                     |
| ----------- | ---------------------------------- | -------------------------------------------------------------------------- |
| Rust        | `//!` or `/*! */`                  | `visit_node` → `line_comment`/`block_comment` starts with `//!`            |
| Python      | `"""..."""` at the beginning       | `visit_node` → `expression_statement` → `string` (first node)              |
| JS/TS/ArkTS | `/** */` at the beginning          | `visit_node` → `comment` starts with `/**`, first sibling                  |
| Java/Kotlin | `/** */` at the beginning          | `visit_node` → `block_comment` starts with `/**`, first sibling            |
| C#          | `/** */` or `///` at the beginning | `visit_node` → `comment` starts with `/**` or `///`                        |
| Swift       | `///` at the beginning             | `visit_node` → `comment` starts with `///`, first or after another comment |
| Dart        | `///` at the beginning             | `visit_node` → `comment` starts with `///`, `is_module_comment`            |
| Go          | `//` at the beginning              | `visit_node` → `comment` starts with `//`, first sibling                   |
| Bash        | `##`                               | `visit_node` → `comment` starts with `##`                                  |
| Batch       | `::`                               | `visit_node` → `comment` starts with `::`, not inside a label              |

### 2. Doc Comments

Describe a specific element (function, class, struct). Stored in the `comments` field of the corresponding object.

**Extraction algorithm:**

1. Take the target node (function, class, etc.)
2. Walk backwards through sibling nodes
3. Collect all doc comments, skipping attributes/annotations
4. Stop at the first non-doc node
5. Reverse the list (from farthest to closest)

**Syntax by language:**

| Language   | Syntax                    | Extraction function                                                                   |
| ---------- | ------------------------- | ------------------------------------------------------------------------------------- |
| Rust       | `///`, `/** */`, `/*! */` | `extract_rust_doc_comments` — walks `prev_sibling`, skips `attribute_item`            |
| Python     | `"""..."""` (docstring)   | `extract_python_docstring` — finds first `string` in `block` → `expression_statement` |
| JavaScript | `/** */` (JSDoc)          | `extract_js_doc_comments` — walks `prev_sibling`                                      |
| TypeScript | `/** */` (JSDoc)          | `extract_ts_doc_comments` — walks `prev_sibling`                                      |
| Java       | `/** */` (Javadoc)        | `extract_java_doc_comments` — walks `prev_sibling`, only `block_comment`              |
| Kotlin     | `/** */` (KDoc)           | `extract_kotlin_doc_comments` — finds `/** */` in prefix before node                  |
| C#         | `///` or `/** */`         | `extract_csharp_doc_comments` — walks `prev_sibling`, skips `attribute_list`          |
| Swift      | `///` or `/** */`         | `extract_swift_doc_comments` — finds `/** */` in prefix before node                   |
| Dart       | `///`                     | `extract_dart_doc_comments` — walks `prev_sibling`                                    |
| Go         | `//`                      | `extract_go_doc_comments` — walks lines in prefix before node                         |
| Bash       | `#` before function       | `extract_bash_doc_comments` — walks `prev_sibling`, functions only                    |
| Batch      | `::` before label         | `extract_batch_doc_comments` — walks `prev_sibling`                                   |
| ArkTS      | `/** */`                  | `extract_arkts_doc_comments` — walks `prev_sibling`, skips `export_declaration`       |

### 3. Regular Comments

Everything else — `//`, `#`, `REM`, `;` — is **ignored by the parser**.

## Signature Extraction

Signatures of functions, classes, and other elements are trimmed at the first delimiter (`{`, `=`, `;`, `:`) — the body is not stored.

**Language-specific details:**

| Language | Detail                                                                                        |
| -------- | --------------------------------------------------------------------------------------------- |
| Python   | Trailing `:` is trimmed from the signature                                                    |
| JS/TS    | Arrow functions — signature is extracted from the variable declaration with `=>`              |
| TS       | `export` prefix is added for exported elements                                                |
| Go       | Methods with receivers (`func (s *Service) Method()`) are detected and extracted as functions |
| Kotlin   | Trimmed at `{` or `=` for expression bodies                                                   |
| Batch    | Functions are `:label`, variables are trimmed from `set `                                     |

## Language-Specific Implementation Details

- **Python** — does not use `child_by_field_name("body")` due to a tree-sitter bug; manual traversal is used instead
- **Swift** — classes, structs, and enums come in a single `class_declaration` node, distinguished by text
- **TypeScript/ArkTS** — doc comments for exported elements are looked up on the parent `export_statement`
- **Java** — methods inside classes are not extracted as separate top-level functions
- **Go** — methods with receivers are detected via `is_method` and are not duplicated
- **Kotlin** — `sealed class` is skipped
- **Batch** — variables inside labels are not extracted
- **Markdown** — frontmatter is parsed manually, heading previews are cleaned of formatting

## AST Export

Results can be exported in several formats:

```bash
# AST only
vibe-analyzer scan ast

# With export
vibe-analyzer scan ast --target my-app --format json5 --output analysis.json5
```

Supported formats: JSON, JSON5, TOON, XML.
