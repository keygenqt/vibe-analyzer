# Supported Languages

Vibe Analyzer supports AST parsing, static analysis, and LLM enrichment for 14 languages.

## Full List

| Language   | Extensions | AST | Static Analysis | Enrichment |
| ---------- | ---------- | --- | --------------- | ---------- |
| Rust       | `.rs`      | ✅  | ✅              | ✅         |
| Python     | `.py`      | ✅  | ✅              | ✅         |
| JavaScript | `.js`      | ✅  | ✅              | ✅         |
| TypeScript | `.ts`      | ✅  | ✅              | ✅         |
| Java       | `.java`    | ✅  | ✅              | ✅         |
| Go         | `.go`      | ✅  | ✅              | ✅         |
| C#         | `.cs`      | ✅  | ✅              | ✅         |
| Kotlin     | `.kt`      | ✅  | ✅              | ✅         |
| Swift      | `.swift`   | ✅  | ✅              | ✅         |
| Dart       | `.dart`    | ✅  | ✅              | ✅         |
| Bash       | `.sh`      | ✅  | ✅              | ✅         |
| Batch      | `.bat`     | ✅  | ✅              | ✅         |
| ArkTS      | `.ets`     | ✅  | ✅              | ✅         |
| Markdown   | `.md`      | ✅  | ✅              | ✅         |

## Extracted Elements

### Code

| Element           | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `functions`       | Functions and methods with signatures and doc comments |
| `classes`         | Class declarations                                     |
| `structs`         | Struct and record declarations                         |
| `enums`           | Enum declarations                                      |
| `interfaces`      | Interface, trait, and protocol declarations            |
| `variables`       | Module-level variables and constants                   |
| `imports`         | Import statements and dependencies                     |
| `header_comments` | File-level documentation comments                      |

### Markdown

| Element       | Description                             |
| ------------- | --------------------------------------- |
| `headings`    | Headings with level, title, and preview |
| `links`       | Links with text and URL                 |
| `code_blocks` | Fenced code block languages             |
| `frontmatter` | YAML frontmatter metadata               |

### Static Analysis

| Element    | Description                                               |
| ---------- | --------------------------------------------------------- |
| `markers`  | Code markers: TODO, FIXME, HACK, XXX, and 17 more         |
| `warnings` | Potential issues: unwrap, panic, empty catch, console.log |

## Doc Comment Formats

Vibe Analyzer extracts documentation from specially formatted comments. Regular comments (`//`, `#`) are ignored.

| Language   | Doc Comment                   | Module Comment                  | Example                                     |
| ---------- | ----------------------------- | ------------------------------- | ------------------------------------------- |
| Rust       | `///` or `/** */`             | `//!` or `/*! */`               | `/// Adds two numbers`                      |
| Python     | `"""..."""` (docstring)       | `"""..."""` at file start       | `"""Adds two numbers"""`                    |
| JavaScript | `/** */` (JSDoc)              | `/** */` at file start          | `/** @param {number} a */`                  |
| TypeScript | `/** */` (JSDoc)              | `/** */` at file start          | `/** @param a First number */`              |
| Java       | `/** */` (Javadoc)            | `/** */` at file start          | `/** @param a First number */`              |
| Kotlin     | `/** */` (KDoc)               | `/** */` at file start          | `/** @param a First number */`              |
| C#         | `///` or `/** */`             | `/** */` or `///` at file start | `/// <summary>Adds two numbers</summary>`   |
| Swift      | `///` or `/** */`             | `///` or `/** */` at file start | `/// - Parameters: a: First number`         |
| Dart       | `///`                         | `///` at file start             | `/// Adds two numbers`                      |
| Go         | `//` (any before declaration) | `//` at file start              | `// Add adds two numbers`                   |
| Bash       | `##` or `#` before function   | `##` at script start            | `## Module documentation for Bash testing`  |
| Batch      | `::` before label             | `::` at script start            | `:: Module documentation for Batch testing` |
| ArkTS      | `/** */`                      | `/** */` at file start          | `/** Async function example */`             |

## Multilingual Search

Each element receives tags in EN, RU, and ZH for language-agnostic search.

## Limitations

- **Maximum file size:** 10 MB
- **Ignored directories:** `target`, `node_modules`, `.git`, `.idea`, etc.
- **Binary files:** excluded from parsing
- **Nested elements:** methods inside classes extracted as functions; variables inside functions are not extracted
