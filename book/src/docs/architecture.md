# Architecture

## Overview

Vibe Analyzer consists of four main components that work sequentially:

```
Code sources
      │
      ▼
┌─────────────┐
│   Scanner   │  AST parsing, structure extraction
└─────────────┘
      │
      ▼
┌─────────────┐
│   Analyzer  │  LLM enrichment, descriptions, tags
└─────────────┘
      │
      ▼
┌─────────────┐
│   Indexer   │  Writing to OpenSearch
└─────────────┘
      │
      ▼
┌─────────────┐
│  MCP Server │  HTTP API for AI assistants
└─────────────┘
```

## Components

### Scanner

The Scanner handles initial source code processing:

- **File system traversal** — recursive directory scanning respecting `.gitignore` and default exclusion patterns (`.git`, `target`, `node_modules`, etc.)
- **Language detection** — selects the appropriate tree-sitter parser based on file extension
- **AST parsing** — extracts code structure: functions, classes, imports, variables, enums, interfaces, structs
- **Metadata collection** — line count, file size, BLAKE3 content hash
- **License detection** — searches for a LICENSE file and identifies the license type via askalono and SPDX
- **README detection** — priority: root > subdirectories, `.md` > `.txt` > no extension
- **Statistics collection** — aggregation by language, file count, lines of code

### Analyzer

The Analyzer enriches scanner results using LLM and static analysis:

- **Static analysis** — finds code markers (TODO, FIXME, HACK) and warnings (unwrap, panic, bare except) across 14 languages
- **LLM enrichment** — four modes: Meta (summaries), Debt (technical debt), Errors (bugs), Advice (refactoring)
- **Prompt generation** — builds a request for each file containing the AST structure
- **Request distribution** — with multiple cluster nodes configured, prompts are distributed across workers via vibe-cluster. Local models (Ollama) get priority, with automatic load balancing across all nodes
- **Batch processing** — files are grouped into batches limited by `max_chunk_chars` (Meta mode). Other modes process one file per prompt
- **Controlled generation** — configurable parameters `temperature`, `seed`, `num_ctx`, `num_predict` for reproducible results
- **Project summarization** — a separate request generates a brief description of the entire source

### Indexer

The Indexer manages writing data to OpenSearch:

- **Three indices** per source:
  - `vibe_meta` — project metadata (summary, license, statistics, README)
  - `vibe_files_{hash}` — full file contents
  - `vibe_files_analysis_{hash}` — AST, enriched descriptions, and search tags
- **Bulk operations** — batch writing for maximum performance
- **Incremental updates** — BLAKE3 hash comparison, only changed files are re-processed
- **Cleanup** — removes stale data no longer present in the source

### MCP Server

The MCP server provides an API for AI assistants:

- **Protocol** — Model Context Protocol (MCP) via Streamable HTTP transport
- **11 tools** — admin, get, search, and show categories
- **Anti-Hallucination Protection** — parameter normalization, tool name aliases, auto language detection
- **Logging** — middleware for tracking all requests
- **CORS** — cross-origin request support for web interfaces

## Indexing Lifecycle

### Full Indexing

```
1. source add → save path to config
2. scan index → check OpenSearch → cleanup orphaned data → AST parsing → LLM enrichment → indexing
```

### Incremental Updates

```
1. scan index → load hashes from OpenSearch → compare with files on disk
2. New/modified → AST parsing → LLM enrichment → indexing
3. Deleted → removal from OpenSearch
4. Unchanged → skip
```

### Export without Indexing

```
1. scan ast → traverse files → AST parsing → export to file
2. scan analyze → traverse files → AST parsing → LLM enrichment → export to file
```

`scan ast` and `scan analyze` do not touch OpenSearch — file export only.

## OpenSearch Indices

### `vibe_meta`

One document per project: summary, license, README, aggregated statistics (files, lines, size).

### `vibe_files_{hash}`

One document per file: full contents. The `content` field is not indexed for search — only stored for retrieval via `get_file_content`.

### `vibe_files_analysis_{hash}`

One document per text file. Contains AST (functions, classes, imports, etc.), file metadata, and multilingual search tags. The `description` and `tags` fields are added after LLM enrichment.

## LLM Cluster

Vibe Analyzer uses [vibe-cluster](https://gitcode.com/keygenqt_vz/vibe-cluster) for distributing enrichment prompts across multiple LLM providers:

- **Multi-provider** — supports Ollama (local), DeepSeek, and Qwen (cloud) simultaneously
- **Load balancing** — prompts are distributed across all available nodes via atomic work-stealing
- **Local-first priority** — local Ollama models get prompts before cloud providers for faster response
- **Parallel connections** — configurable `parallel` parameter for multiple workers per provider
- **Automatic retry** — network errors and server failures trigger automatic retries
- **Exclusive mode** — eject and reload models between prompts for clean context
- **Per-node statistics** — after enrichment, reports how many files each node processed

This approach allows:

- **Faster enrichment** through parallel processing on multiple GPUs and cloud APIs
- **Cost optimization** — local models handle most work, cloud used only when needed
- **Scaling** — add more nodes to the cluster configuration as needed

## Anti-Hallucination Protection

Protection against AI model hallucinations when calling tools:

| Mechanism                   | Description                                                                              |
| --------------------------- | ---------------------------------------------------------------------------------------- |
| **Name aliases**            | 150+ alternative tool names (e.g., `search_code_functions` → `search_by_code_functions`) |
| **Parameter normalization** | Wildcard replacement, whitespace trimming, type casting                                  |
| **Bounds validation**       | `limit` always in 1–10 range, `level` capped                                             |
| **Auto language detection** | Detects Cyrillic, Latin, and CJK in search queries                                       |
| **Soft error handling**     | Invalid parameters don't cause errors, they are normalized to safe values                |

## Performance

- **Rust** — native execution without GC overhead
- **Parallel parsing** — each file processed independently
- **Bulk OpenSearch writes** — thousands of documents per operation
- **Streaming processing** — files are processed as they are discovered, without waiting for the entire directory
- **Incremental updates** — only changed files are re-indexed when updating a source
