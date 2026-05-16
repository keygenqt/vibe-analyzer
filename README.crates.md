# Vibe Analyzer

Codebase analysis and Agentic RAG engine. AST extraction, LLM enrichment, static analysis, OpenSearch indexing.
11 MCP tools give AI models agency over search — they decide what to find and how.

## Why Agentic RAG

Any RAG adds found documents to the prompt — larger context = more VRAM.
Vibe Analyzer returns structured results via tools, keeping context as clean as possible:

- 📉 Minimal context — model receives only what the tool returns
- 🧠 No embeddings — keyword and AST search via OpenSearch
- 🔗 Single tool call = complete answer, no document stuffing
- ♾️ Context stays small regardless of project size

## Preview

![preview](https://raw.gitcode.com/keygenqt_vz/vibe-analyzer/files/main/data/preview/preview-webui.png)

## Features

- 🌳 AST parsing for 14 languages
- 🔍 Static analysis: code markers (TODO, FIXME, HACK), warnings (unwrap, panic)
- 💡 LLM enrichment: technical debt, bugs, refactoring suggestions, file summaries
- 📄 Export AST and analysis results to JSON, JSON5, TOML, TOON, XML, YAML
- 📝 Semantic and morphological search across code and knowledge base
- ⚡ Incremental indexing
- 📦 Self-contained MCP tools (single call — complete response)
- 🗂️ Multilingual support (EN, RU, CN)
- 🦀 Built in Rust — fast and memory-efficient

## Anti-Hallucination Protection

- ✅ Soft parameter validation
- 🛡️ Input parameter normalization
- 📋 Tool description optimization
- 🏷️ Tool name aliases (150+)
- 🌐 Automatic language detection
- 🧪 End-to-end testing with full cycle
- 📐 Tested on models from 3B+

## Supported Languages

| Language   | AST | Enrichment |
| ---------- | --- | ---------- |
| Rust       | ✅  | ✅         |
| Python     | ✅  | ✅         |
| JavaScript | ✅  | ✅         |
| TypeScript | ✅  | ✅         |
| Java       | ✅  | ✅         |
| Go         | ✅  | ✅         |
| C#         | ✅  | ✅         |
| Kotlin     | ✅  | ✅         |
| Swift      | ✅  | ✅         |
| Dart       | ✅  | ✅         |
| Bash       | ✅  | ✅         |
| Batch      | ✅  | ✅         |
| ArkTS      | ✅  | ✅         |
| Markdown   | ✅  | ✅         |

## Analyze Tools

| Tool     | Description                                                          |
| -------- | -------------------------------------------------------------------- |
| `ast`    | AST parsing only — code structure without LLM enrichment             |
| `meta`   | Generate summary and search tags for files                           |
| `debt`   | Detect technical debt: TODO markers, magic numbers, code duplication |
| `errors` | Find bugs: unsafe calls, swallowed errors, spelling mistakes         |
| `advice` | Suggest refactoring, naming improvements, and missing tests          |

## MCP Tools

| Tool                       | Description                                                                |
| -------------------------- | -------------------------------------------------------------------------- |
| `admin_sync`               | Reindex all projects                                                       |
| `get_file_content`         | Full file content by path (supports partial/wildcard matching)             |
| `get_file_ast`             | Full AST: imports, functions, classes, structs, enums, headers             |
| `search_by_code_imports`   | Search by imports/dependencies                                             |
| `search_by_code_functions` | Search by functions/methods                                                |
| `search_by_code_classes`   | Search by classes/structs/interfaces                                       |
| `search_by_code_variables` | Search by variables/constants/enums                                        |
| `search_documentation`     | Search all markdown docs (legends, guides, rules). Multilingual (RU/EN/ZH) |
| `search_knowledge`         | Alias for documentation search                                             |
| `show_projects`            | List all indexed projects                                                  |
| `show_stats`               | Stats by language, files, lines of code                                    |
| `show_tree`                | Directory tree of files and folders                                        |

## Quick Start

```bash
# 1. Install
cargo install vibe-analyzer

# 2. Add a project or documentation folder
vibe-analyzer source add <path>

# 3. Edit config (~/.vibe-analyzer/config.json5)
#    Set OpenSearch host and Ollama model

# 4. Export analysis (AST + LLM enrichment)
vibe-analyzer analyze export -m meta,debt,errors -f json5

# 5. Start MCP server for AI assistants
vibe-analyzer serve start
```

## CLI Reference

```bash
# Source management
vibe-analyzer source add <path>                     # Add knowledge source
vibe-analyzer source remove --target <path>         # Remove source
vibe-analyzer source list                           # List all sources

# Analysis and export
vibe-analyzer analyze export                        # AST only, all sources
vibe-analyzer analyze export -m meta,debt           # With LLM enrichment
vibe-analyzer analyze export -t code -f json5       # Code only, JSON5 format
vibe-analyzer analyze export -m errors -t markdown  # Errors in docs only

# Stats
vibe-analyzer stats info [--target <name>]          # Show project statistics
vibe-analyzer stats tree --target <name> [-L <n>]   # Show directory tree

# MCP Server
vibe-analyzer serve start                           # Start MCP server
```

## Dependencies

- OpenSearch
- Ollama (for LLM enrichment)
- libssl-dev
