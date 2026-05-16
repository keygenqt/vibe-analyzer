# Quick Start

## Prerequisites

- **OpenSearch** — storage and search for indexed data
- **Ollama** — running LLMs for code enrichment

Both can be run locally via Docker.

## Installation

```bash
# Via Cargo (recommended)
cargo install vibe-analyzer

# Or build from source
git clone https://gitcode.com/keygenqt_vz/vibe-analyzer.git
cd vibe-analyzer
cargo build --release
```

## Starting Services

```bash
# OpenSearch
cd docker/opensearch && docker-compose up -d

# Open WebUI (optional, for AI assistant connection)
cd docker/open-webui && docker-compose up -d
```

## Configuration

Config file: `~/.vibe-analyzer/config.json5` (created automatically on first run).

See [Configuration](./configuration.md) for all options and detailed reference.

## Usage

```bash
# Add a source
vibe-analyzer source add /path/to/project

# AST only (no LLM)
vibe-analyzer analyze export

# AST + LLM enrichment
vibe-analyzer analyze export -m meta,debt,errors

# Code files only, JSON5 format
vibe-analyzer analyze export -m debt -t code -f json5

# Index to OpenSearch
vibe-analyzer analyze index --target my-project

# Start MCP server for AI assistants
vibe-analyzer serve start

# View statistics
vibe-analyzer stats info
```

## What's Next

- [Architecture](./architecture.md) — detailed breakdown
- [CLI Reference](./cli-reference.md) — complete command reference
- [Configuration](./configuration.md) — all config options
- [Integrations](./integrations.md) — connecting AI assistants
