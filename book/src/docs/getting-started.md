## Chapter 2. Getting Started

### Prerequisites

Before installing Vibe Analyzer, you need two services running:

| Dependency     | Purpose                           | Minimum Version |
| -------------- | --------------------------------- | --------------- |
| **OpenSearch** | Stores and searches indexed code  | 2.x             |
| **Ollama**     | LLM inference for code enrichment | Latest          |

**OpenSearch** is the search engine. It stores parsed code structures and responds to MCP tool queries.

**Ollama** runs local language models. It generates descriptions and multilingual tags for your code during enrichment.

### Installation

```bash
# Clone the repository
git clone https://gitcode.com/keygenqt_vz/vibe-analyzer.git

# Enter the project directory
cd vibe-analyzer

# Build the release binary
cargo build --release
```

The compiled binary is at `target/release/vibe-analyzer`. Move it to your PATH or use it directly.

### Starting Required Services

**Option 1: Docker (recommended)**

The project includes Docker Compose files:

```bash
# Start OpenSearch
docker compose -f docker/opensearch/docker-compose.yml up -d

# Verify OpenSearch is running
curl http://localhost:9200

# Start Ollama
ollama serve

# Pull a model for enrichment
ollama pull qwen2.5:14b
```

**Option 2: Manual installation**

Install OpenSearch and Ollama manually following their official documentation.
Open WebUI is optional but recommended — it provides a chat interface for AI assistants.

```bash
# Optional: Start Open WebUI
docker compose -f docker/open-webui/docker-compose.yml up -d
```

### Configuration

The config file is at `~/.vibe-analyzer/config.json5`. Create it manually or run any CLI command — it will generate a default config.

```json
{
  // OpenSearch connection
  "opensearch": {
    "host": "http://localhost:9200"
  },
  // Ollama connection (supports multiple hosts for load balancing)
  "ollama": {
    "hosts": ["http://localhost:11434"],
    "model": "qwen2.5:14b",
    // Maximum characters per LLM request
    "max_chunk_chars": 4200
  },
  // MCP server configuration
  "mcp": {
    "host": "0.0.0.0",
    "port": 3001
  },
  // Development settings (optional)
  // dev: {
  //   log_level: "info",
  // },
  // Your knowledge sources
  "sources": []
}
```

Key settings:

- `opensearch.host` — your OpenSearch endpoint
- `ollama.hosts` — array of Ollama instances (single or cluster)
- `ollama.model` — model used for enrichment (3B+ models recommended)
- `mcp.host` / `mcp.port` — where the MCP server listens

### Adding Your First Source

A **source** is anything you want to search: a code project, a documentation folder, or both.

```bash
# Add a project to analyze
vibe-analyzer source add /path/to/your/project

# Add a documentation folder
vibe-analyzer source add /path/to/your/docs

# Verify sources are added
vibe-analyzer source list
```

### Scanning and Indexing

Processing happens in two stages:

**Stage 1: AST Extraction**

Parses all source files and extracts code structure — functions, classes, imports, variables, and more.

```bash
# Scan all sources
vibe-analyzer scan ast

# Or scan a specific source
vibe-analyzer scan ast --target /path/to/your/project
```

**Stage 2: LLM Enrichment + Indexing**

Sends extracted AST to Ollama for enrichment, then indexes everything into OpenSearch.

```bash
# Enrich and index a specific source
vibe-analyzer scan analyze --target /path/to/your/project
```

The `analyze` command does three things:

1. Sends code structures to Ollama in batches
2. Receives descriptions and multilingual tags
3. Indexes enriched data into OpenSearch

Duration depends on project size and Ollama model speed. Large projects may take minutes.

### Starting the MCP Server

Once indexed, start the MCP server to expose search tools:

```bash
# Start MCP server
vibe-analyzer serve start
```

The server runs on the configured host:port. It exposes 11 tools for AI assistants.

### Verifying Everything Works

```bash
# Check project statistics
vibe-analyzer stats info --target /path/to/your/project

# View directory tree (2 levels deep)
vibe-analyzer stats tree --target /path/to/your/project -L 2

# List all indexed projects
vibe-analyzer stats info
```

Expected output shows file counts, lines of code, and language breakdown.

### Connecting an AI Assistant

**Open WebUI:**

1. Open Open WebUI → Settings → MCP Servers
2. Add server: `http://localhost:3001`
3. Start a new chat — the model now has access to 11 Vibe Analyzer tools

**Claude Desktop:**

1. Open Claude → Settings → Developer → MCP Servers
2. Add server with URL: `http://localhost:3001`
3. Tools appear in the chat interface automatically

### Incremental Updates

Vibe Analyzer uses content hashing for incremental indexing. Only modified files are re-processed:

```bash
# Re-index only changed files across all sources
vibe-analyzer scan analyze
```

Or trigger re-indexing through MCP:

```
AI Assistant: "Sync all projects"
→ Tool: admin_sync
→ Result: 15 new files indexed, 3 files updated
```

### Troubleshooting

**OpenSearch not reachable:**

```bash
curl http://localhost:9200
# If no response, check: docker ps | grep opensearch
```

**Ollama not responding:**

```bash
curl http://localhost:11434/api/tags
# If no response, run: ollama serve
```

**Model not found:**

```bash
ollama list
# If model is missing: ollama pull qwen2.5:14b
```

**Config not found:**

```bash
# Generate default config by running any command
vibe-analyzer source list
# Config created at ~/.vibe-analyzer/config.json5
```

### What's Next

- [Chapter 3: CLI Reference](./cli-reference.md) — complete command reference
- [Chapter 5: MCP Tools](./mcp-tools.md) — how to use all 11 search tools
- [Chapter 6: Configuration](./configuration.md) — advanced configuration options
