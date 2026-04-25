# Vibe Analyzer

Agentic RAG engine for code and knowledge bases. AST extraction, LLM enrichment, OpenSearch indexing.
11 MCP tools give AI models agency over search — they decide what to find and how.

## Why Agentic RAG

Unlike traditional RAG where documents are injected into context automatically,
Vibe Analyzer gives the model agency over search:

- 🎯 Model decides which tool to call (code, docs, or file)
- 📊 Structured results — no context bloat
- 🧠 Model chooses target, limit, and language per query
- 🔗 Single tool call = complete answer

## Features

- 🌳 AST parsing for 13 languages
- 💡 AST enrichment via LLM (descriptions, tags)
- 📄 Export AST and AST+LLM to JSON, JSON5, TOON, XML
- 📝 Semantic and morphological search across code and knowledge base
- ⚡ Incremental indexing
- 📦 Self-contained tools (single call — complete response)
- 🗂️ Multilingual support (EN, RU, ZH)
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

## MCP Tools

### Admin

| Tool         | Description          |
| ------------ | -------------------- |
| `admin_sync` | Reindex all projects |

### Get

| Tool               | Description                                                    |
| ------------------ | -------------------------------------------------------------- |
| `get_file_content` | Full file content by path (supports partial/wildcard matching) |
| `get_file_ast`     | Full AST: imports, functions, classes, structs, enums, headers |

### Search — Code

| Tool                       | Description                          |
| -------------------------- | ------------------------------------ |
| `search_by_code_imports`   | Search by imports/dependencies       |
| `search_by_code_functions` | Search by functions/methods          |
| `search_by_code_classes`   | Search by classes/structs/interfaces |
| `search_by_code_variables` | Search by variables/constants/enums  |

### Search — Docs

| Tool                   | Description                                                                |
| ---------------------- | -------------------------------------------------------------------------- |
| `search_documentation` | Search all markdown docs (legends, guides, rules). Multilingual (RU/EN/ZH) |
| `search_knowledge`     | Alias for documentation search                                             |

### Show

| Tool            | Description                             |
| --------------- | --------------------------------------- |
| `show_projects` | List all indexed projects               |
| `show_stats`    | Stats by language, files, lines of code |
| `show_tree`     | Directory tree of files and folders     |

## Quick Start

```bash
# 1. Add knowledge sources
vibe-analyzer source add <path>

# 2. Edit config if needed (~/.vibe-analyzer/config.json5)
#    Set your OpenSearch host and Ollama model

# 3. Scan and index with LLM enrichment
vibe-analyzer scan analyze --target <name>

# 4. Start MCP server for AI assistants
vibe-analyzer serve start
```

## CLI Reference

```bash
# Source management
vibe-analyzer source add <path>                     # Add knowledge source
vibe-analyzer source remove --target <path>         # Remove source
vibe-analyzer source list                           # List all sources

# Scanning
vibe-analyzer scan ast                              # All sources (AST only)
vibe-analyzer scan ast --target <name>              # Specific source
vibe-analyzer scan analyze --target <name>          # With LLM enrichment

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
