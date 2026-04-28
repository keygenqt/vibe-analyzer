# Quick Start

## Prerequisites

Vibe Analyzer requires two external services:

- **OpenSearch** — storage and search for indexed data
- **Ollama** — running LLMs for code enrichment with descriptions and tags

Both services can be run locally via Docker.

## Installation

### Install via Cargo (recommended)

The package is available on [crates.io](https://crates.io/crates/vibe-analyzer):

```bash
cargo install vibe-analyzer
```

### Build from Source

```bash
# 1. Clone the repository
git clone https://gitcode.com/keygenqt_vz/vibe-analyzer.git

# 2. Enter the directory
cd vibe-analyzer

# 3. Build
cargo build --release
```

### Build Dependencies

- Rust toolchain (cargo, rustc)
- libssl-dev (for TLS)

## Starting Services

The repository includes two ready-to-use docker-compose files:

- `docker/opensearch/docker-compose.yml` — OpenSearch for indexing and search
- `docker/open-webui/docker-compose.yml` — Open WebUI with Ollama for AI assistant connection

### Docker (recommended)

**OpenSearch:**

```bash
cd docker/opensearch
docker-compose up -d
```

**Open WebUI (optional, for AI assistant connection):**

```bash
cd docker/open-webui
docker-compose up -d
```

### Verification

```bash
# OpenSearch should respond
curl http://localhost:9200

# Ollama should be accessible
curl http://localhost:11434/api/tags
```

## Configuration

The configuration file is located at `~/.vibe-analyzer/config.json5`. It is created automatically with default settings the first time any CLI command is run.

### Example Working Configuration

```json
{
  // Configuration version (do not modify)
  "version": "0.0.1",

  // OpenSearch connection
  "opensearch": {
    "host": "http://192.168.1.10:9200"
  },

  // MCP server
  //
  // host — bind address (0.0.0.0 for all interfaces, 127.0.0.1 local only)
  // port — server port (default: 9020)
  // protocol — MCP protocol version (2024-11-05, 2025-03-26, 2025-06-18, or 'latest')
  "mcp": {
    "host": "0.0.0.0",
    "port": 9020,
    "protocol": "latest"
  },

  // Ollama LLM servers
  //
  // host — API endpoint
  // model — model name
  // max_chunk_chars — maximum characters per request
  // max_chunk_files — maximum files per request
  // timeout_secs — request timeout in seconds
  // temperature — generation temperature (0.0 – 1.0)
  // seed — seed for reproducibility
  // num_ctx — context window size
  // num_predict — maximum tokens in response
  // ast_imports — include imports in analysis
  // ast_variables — include variables in analysis
  // ast_functions — include functions in analysis
  // ast_enums — include enums in analysis
  // ast_interfaces — include interfaces in analysis
  "ollama": [
    {
      "host": "http://192.168.1.10:11434",
      "model": "qwen2.5-coder:3b-instruct",
      "max_chunk_chars": 4000,
      "max_chunk_files": 3,
      "timeout_secs": 60,
      "temperature": 0.1,
      "seed": 42,
      "num_ctx": 4096,
      "num_predict": 2048,
      "ast_imports": false,
      "ast_variables": false,
      "ast_functions": true,
      "ast_enums": true,
      "ast_interfaces": true
    },
    {
      "host": "http://localhost:11434",
      "model": "qwen2.5-coder:3b-instruct",
      "max_chunk_chars": 4000,
      "max_chunk_files": 3,
      "timeout_secs": 60,
      "temperature": 0.1,
      "seed": 42,
      "num_ctx": 4096,
      "num_predict": 2048,
      "ast_imports": false,
      "ast_variables": false,
      "ast_functions": true,
      "ast_enums": true,
      "ast_interfaces": true
    }
  ],

  // Knowledge sources for indexing
  "sources": ["/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer"]
}
```

### Checking Configuration

```bash
# View current settings
cat ~/.vibe-analyzer/config.json5
```

## Adding a Knowledge Source

A source is anything you want to index: a code project, a documentation folder, or both.

```bash
# Add a project
vibe-analyzer source add /path/to/your/project

# Add a documentation directory
vibe-analyzer source add /path/to/docs

# List all added sources
vibe-analyzer source list
```

## Scanning and Indexing

Vibe Analyzer provides three commands for different tasks:

### Export AST to File

Code structure extraction only, without LLM. The result is saved in JSON/JSON5/TOON/XML:

```bash
# All sources
vibe-analyzer scan ast

# A specific source
vibe-analyzer scan ast --target /path/to/your/project

# With format specified
vibe-analyzer scan ast --target /path/to/your/project --format json5
```

### Export AST with LLM Enrichment to File

AST parsing + enrichment via Ollama (descriptions, tags). The result is saved to a file:

```bash
vibe-analyzer scan analyze --target /path/to/your/project
```

> **Note:** enrichment requires a running Ollama with the selected model. If multiple Ollama hosts are configured, files are distributed among them via competing consumers.

### Indexing to OpenSearch

Full cycle — AST parsing, LLM enrichment, and writing to OpenSearch for search via MCP tools:

```bash
vibe-analyzer scan index --target /path/to/your/project
```

After indexing, data is ready for search through the MCP server.

## Starting the MCP Server

```bash
vibe-analyzer serve start
```

The server starts on the address and port specified in the configuration (default `http://0.0.0.0:9020`).

## Verifying Results

```bash
# Project statistics
vibe-analyzer stats info --target /path/to/your/project

# File tree
vibe-analyzer stats tree --target /path/to/your/project

# List all indexed projects
vibe-analyzer stats info
```

## Connecting an AI Assistant

### Open WebUI

1. Make sure Open WebUI is running (see the "Starting Services" section)
2. In Open WebUI settings, add an MCP server:
   - URL: `http://<host>:9020` (as specified in the configuration)
   - Transport: Streamable HTTP
3. Once connected, the AI model will have 11 tools for searching code and documentation

## Incremental Updates

Vibe Analyzer uses BLAKE3 hashes to track changes. When running `scan index` again, only modified files are processed:

```bash
# Reindexing — only changed files are affected
vibe-analyzer scan index --target /path/to/your/project
```

To force a full reindex, use the `--force` flag:

```bash
vibe-analyzer scan index --target /path/to/your/project --force
```

The same can be done via the `admin_sync` MCP tool without restarting the server.

## Troubleshooting

### OpenSearch Unreachable

```bash
# Check Docker container status
docker ps | grep opensearch
```

### Ollama Not Responding

```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags
```

### Model Not Installed

```bash
# Download the model
ollama pull qwen2.5-coder:3b-instruct
```

## What's Next

- [Architecture](./architecture.md) — detailed breakdown of Vibe Analyzer's internals
- [CLI Reference](./cli-reference.md) — complete command reference
- [Integrations](./integrations.md) — connecting AI assistants
