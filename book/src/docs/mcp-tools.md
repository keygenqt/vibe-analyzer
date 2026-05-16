# MCP Tools

Vibe Analyzer provides 11 MCP tools that AI models can call to search code and documentation. Each tool returns a structured response — no document stuffing.

## How It Works

```
AI model → selects a tool → calls MCP → receives a structured response
```

**Rules for the AI model:** use tools, respond concisely, one call is enough.

## Tools

| Tool                       | Parameters                                | Description                                                        |
| -------------------------- | ----------------------------------------- | ------------------------------------------------------------------ |
| `admin_sync`               | —                                         | Reindex all projects in the background                             |
| `get_file_content`         | `path` (required)                         | Full file content by path. Supports partial matching and wildcards |
| `get_file_ast`             | `path` (required)                         | Full AST: imports, functions, classes, structs, enums, headings    |
| `search_by_code_imports`   | `query` (required), `target`, `limit` (3) | Search by imports/dependencies                                     |
| `search_by_code_functions` | `query` (required), `target`, `limit` (3) | Search by functions/methods                                        |
| `search_by_code_classes`   | `query` (required), `target`, `limit` (3) | Search by classes/structs/interfaces                               |
| `search_by_code_variables` | `query` (required), `target`, `limit` (3) | Search by variables/constants/enums                                |
| `search_documentation`     | `query` (required), `limit` (3)           | Search all markdown docs. Multilingual (RU/EN/ZH)                  |
| `search_knowledge`         | `query` (required), `limit` (3)           | Alias for `search_documentation`                                   |
| `show_projects`            | `size` (10)                               | List all indexed projects with descriptions                        |
| `show_stats`               | `target`                                  | Project statistics by language, files, lines of code               |
| `show_tree`                | `target`, `level` (3)                     | Directory tree of files and folders                                |

All `limit` parameters are capped to 1–10 range. `target` accepts full path or unique directory name.

## Anti-Hallucination Protection

| Mechanism               | Description                                               |
| ----------------------- | --------------------------------------------------------- |
| Name aliases (160+)     | Fixes distorted tool names automatically                  |
| Parameter normalization | Wildcards, whitespace, invalid values → safe defaults     |
| Auto language detection | Cyrillic → Russian tags, Latin → English, CJK → Chinese   |
| Soft error handling     | Invalid parameters don't cause errors, they're normalized |
