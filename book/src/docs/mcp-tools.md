# MCP Tools

Vibe Analyzer provides 11 MCP tools that AI models can call to search code and documentation. Each tool returns a structured response — no document stuffing into the context.

## General Concept

In traditional RAG, a search engine finds documents and adds them to the prompt. Vibe Analyzer works differently:

```
AI model → selects a tool → calls MCP → receives a structured response
```

**Rules for the AI model (embedded in `ServerInfo.instructions`):**

1. Use tools, respond concisely
2. **One call is enough** — no need to call multiple tools in sequence
3. Only the listed tools

## Tool Categories

| Category          | Tools                                                                                                      | Purpose                                             |
| ----------------- | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| **Admin**         | `admin_sync`                                                                                               | Reindex all projects                                |
| **Get**           | `get_file_content`, `get_file_ast`                                                                         | Retrieve file contents and AST                      |
| **Show**          | `show_projects`, `show_stats`, `show_tree`                                                                 | Project info: list, statistics, file tree           |
| **Search — Code** | `search_by_code_imports`, `search_by_code_functions`, `search_by_code_classes`, `search_by_code_variables` | Code search: imports, functions, classes, variables |
| **Search — Docs** | `search_documentation`, `search_knowledge`                                                                 | Markdown documentation and knowledge base search    |

---

## Admin

### admin_sync

Triggers reindexing of all projects in the background.

**When to call:** the user says "update", "sync", "reindex", "refresh".

**Parameters:** none.

**Response:**

```json
{
  "result": "Started",
  "message": "Indexing started. Projects are updating now."
}
```

Or, if indexing is already running:

```json
{
  "result": "AlreadyRunning",
  "message": "Indexing is already running. Please wait."
}
```

---

## Get

### get_file_content

Returns the full contents of a file.

**When to call:** the user asks to see file contents, open a file.

**Parameters:**

| Parameter | Type   | Required | Description                                                                     |
| --------- | ------ | -------- | ------------------------------------------------------------------------------- |
| `path`    | string | yes      | File path. Supports partial matching and wildcards. Can be relative or absolute |

**Response:**

```json
{
  "path": "src/main.rs",
  "content": "fn main() {\n println!(\"Hello\");\n}\n"
}
```

### get_file_ast

Returns the full AST of a file: imports, functions, classes, structs, enums, headings.

**When to call:** the user asks about file structure, functions in a file, AST.

**Parameters:**

| Parameter | Type   | Required | Description                            |
| --------- | ------ | -------- | -------------------------------------- |
| `path`    | string | yes      | File path. Can be relative or absolute |

**Response:**

```json
{
  "path": "src/main.rs",
  "language": "Rust",
  "size_bytes": 2122,
  "header_comments": ["Vibe Analyzer - Main entry point."],
  "imports": ["clap::Parser", "crate::cli::scan::ScanAction"],
  "functions": [{ "signature": "async fn main()", "comments": [] }],
  "structs": [{ "signature": "struct App", "comments": [] }],
  "enums": [{ "signature": "enum Commands", "comments": [] }]
}
```

---

## Show

### show_projects

Shows all indexed projects with names and brief descriptions.

**When to call:** the user asks "what projects are available", "list projects".

**Parameters:**

| Parameter | Type    | Required | Default | Description      |
| --------- | ------- | -------- | ------- | ---------------- |
| `size`    | integer | no       | 10      | Maximum projects |

**Response:**

```json
{
  "projects": [
    {
      "path": "/path/to/project",
      "name": "vibe-analyzer",
      "summary": "Agentic RAG engine for code and knowledge bases"
    }
  ],
  "total": 1
}
```

### show_stats

Shows project statistics: language breakdown, file count, lines of code, AST objects.

**When to call:** the user asks about statistics, file count, codebase size.

**Parameters:**

| Parameter | Type   | Required | Description                                                  |
| --------- | ------ | -------- | ------------------------------------------------------------ |
| `target`  | string | no       | Project name. If not specified — statistics for all projects |

**Response:**

```json
{
  "languages": [
    {
      "language": "Rust",
      "files": 194,
      "lines": 15146,
      "ast_objects": 1631,
      "size_human": "498.26 KB"
    },
    {
      "language": "Markdown",
      "files": 33,
      "lines": 2884,
      "ast_objects": 296,
      "size_human": "102.64 KB"
    }
  ],
  "total": { "files": 287, "lines": 24893, "ast_objects": 2082, "size_human": "755.11 KB" }
}
```

### show_tree

Shows the file and directory tree of a project.

**When to call:** the user asks about project structure, file tree, folders.

**Parameters:**

| Parameter | Type    | Required | Default      | Description          |
| --------- | ------- | -------- | ------------ | -------------------- |
| `target`  | string  | no       | all projects | Project name         |
| `level`   | integer | no       | `3`          | Maximum depth (1–10) |

**Response:**

```json
{
  "tree": "vibe-analyzer\n|-- Cargo.toml\n|-- src\n|   |-- main.rs\n|   `-- utils\n|       `-- ...\n`-- tests\n    `-- ...",
  "files": 74,
  "directories": 36
}
```

---

## Search — Code

### search_by_code_imports

Finds imports and dependencies in code.

**When to call:** the user asks about imports, dependencies, libraries used.

**Parameters:**

| Parameter | Type    | Required | Default      | Description     |
| --------- | ------- | -------- | ------------ | --------------- |
| `query`   | string  | yes      | —            | Search query    |
| `target`  | string  | no       | all projects | Project name    |
| `limit`   | integer | no       | `3`          | Maximum results |

**Response:**

```json
{
  "query": "serde",
  "results": [
    {
      "path": "src/main.rs",
      "language": "Rust",
      "header_comments": ["Vibe Analyzer - Main entry point."],
      "imports": ["serde::Deserialize", "serde::Serialize"]
    }
  ]
}
```

### search_by_code_functions

Finds functions and methods in code.

**When to call:** the user asks about functions, methods, procedures.

**Parameters:**

| Parameter | Type    | Required | Default      | Description     |
| --------- | ------- | -------- | ------------ | --------------- |
| `query`   | string  | yes      | —            | Search query    |
| `target`  | string  | no       | all projects | Project name    |
| `limit`   | integer | no       | `3`          | Maximum results |

**Response:**

```json
{
  "query": "scan_source",
  "results": [
    {
      "path": "src/scanner/scanner.rs",
      "language": "Rust",
      "header_comments": ["Core scanning functionality for codebase analysis."],
      "functions": [
        {
          "signature": "pub async fn scan_source(...)",
          "comments": ["Scans a source and returns complete analysis results"]
        }
      ]
    }
  ]
}
```

### search_by_code_classes

Finds classes, structs, interfaces, and traits.

**When to call:** the user asks about classes, structs, interfaces, types, traits.

**Parameters:**

| Parameter | Type    | Required | Default      | Description     |
| --------- | ------- | -------- | ------------ | --------------- |
| `query`   | string  | yes      | —            | Search query    |
| `target`  | string  | no       | all projects | Project name    |
| `limit`   | integer | no       | `3`          | Maximum results |

**Response:**

```json
{
  "query": "AppConfig",
  "results": [
    {
      "path": "src/configs/app.rs",
      "language": "Rust",
      "header_comments": ["Application configuration management for vibe-analyzer."],
      "classes": [],
      "structs": [
        {
          "signature": "pub struct AppConfig",
          "comments": ["Main application configuration structure"]
        }
      ],
      "interfaces": []
    }
  ]
}
```

### search_by_code_variables

Finds variables, constants, and enums.

**When to call:** the user asks about variables, constants, enums, static fields.

**Parameters:**

| Parameter | Type    | Required | Default      | Description     |
| --------- | ------- | -------- | ------------ | --------------- |
| `query`   | string  | yes      | —            | Search query    |
| `target`  | string  | no       | all projects | Project name    |
| `limit`   | integer | no       | `3`          | Maximum results |

**Response:**

```json
{
  "query": "MAX_SIZE",
  "results": [
    {
      "path": "src/utils/constants.rs",
      "language": "Rust",
      "header_comments": ["Application constants and configuration defaults."],
      "variables": [
        {
          "signature": "pub const MAX_AST_FILE_SIZE: u64",
          "comments": ["Maximum file size for AST parsing (10 MB)"]
        }
      ],
      "enums": []
    }
  ]
}
```

---

## Search — Docs

### search_documentation

Searches all Markdown documentation files. This is the **default** tool for non-code questions.

**When to call:** "who is", "what is", "how does", "rules", "processes", "guides", "legends" questions.

**Search priority:** Markdown files with `knowledge: true` in the frontmatter receive a significant boost (5.0) and appear first. This separates the knowledge base (legends, guidelines) from regular documentation. Example frontmatter:

```yaml
---
knowledge: true
---
```

**Parameters:**

| Parameter | Type    | Required | Default | Description                                 |
| --------- | ------- | -------- | ------- | ------------------------------------------- |
| `query`   | string  | yes      | —       | Search query. Supports Cyrillic, Latin, CJK |
| `limit`   | integer | no       | `3`     | Maximum results                             |

**Response:**

```json
{
  "query": "architecture",
  "results": [
    {
      "path": "docs/architecture.md",
      "frontmatter": { "title": "Vibe Analyzer Architecture" },
      "headings": [
        { "level": 1, "title": "Architecture", "preview": "Overview of Vibe Analyzer's design" }
      ],
      "links": [{ "text": "Quick Start", "url": "./getting-started.md" }],
      "code_blocks": ["bash", "rust"]
    }
  ]
}
```

### search_knowledge

Alias for `search_documentation`. Completely identical in parameters and response.

---

## Anti-Hallucination Protection

### Tool Name Aliases (160+)

Models often distort tool names. `AliasHandler` intercepts the call and replaces the name with the correct one:

```
ALIAS_HANDLER: Resolving 'search_functions' -> 'search_by_code_functions'
```

### Parameter Normalization

| Mechanism              | Example                                                      |
| ---------------------- | ------------------------------------------------------------ |
| Wildcard replacement   | `*` and `?` in query → space                                 |
| Whitespace trimming    | `" search query "` → `"search query"`                        |
| `query: "*"` handling  | Returns `None` (all elements)                                |
| `limit` capping        | Always in 1–10 range. Values ≤ 1 → default (3)               |
| Fuzzy path matching    | Partial match and wildcards for `path` in `get_file_content` |
| `target` normalization | Search by exact path or unique directory name                |

### Auto Language Detection

When searching documentation, the system detects scripts in the query:

- **Cyrillic** → search using Russian tags
- **Latin** → search using English tags
- **CJK** → search using Chinese tags

Mixed queries search across all detected scripts simultaneously.

### Soft Error Handling

Invalid parameters don't cause errors, they are normalized to safe values:

- Invalid `target` → search across all projects
- `limit` > 10 → capped to 10
- Non-existent `path` → returns an empty result, not an error
