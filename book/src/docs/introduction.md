# Introduction

## What is Vibe Analyzer

Vibe Analyzer is an Agentic RAG engine for codebases and knowledge bases. It extracts structure from source code via AST parsing, enriches it with LLM, and indexes everything into OpenSearch. AI assistants access knowledge through 11 MCP tools.

## The Problem with Traditional RAG

Traditional RAG works like this:

```
Query → Embeddings → Find similar documents → Load into prompt → Response
```

**Problems:**

- 📈 Found documents are added to the prompt in their entirety
- 💾 The larger the project, the more VRAM is required
- 🔍 Relevance drops as context volume grows
- 💸 Each query becomes more expensive

## How Agentic RAG Works

Vibe Analyzer flips the paradigm:

```
Query → AI model selects an MCP tool → Tool returns a structured response
```

**Advantages:**

- 📉 Minimal context — the model receives only what the tool returns
- 🧠 No embeddings — keyword and AST search via OpenSearch
- 🔗 One tool call = complete answer, no document stuffing
- ♾️ Context size stays constant regardless of project size

## Key Features

- 🌳 AST parsing for 13 programming languages
- 💡 LLM enrichment: descriptions and search tags for each file
- 📄 Export AST and AST+LLM to JSON, JSON5, TOON, XML
- 📝 Semantic and morphological search across code and documentation
- ⚡ Incremental indexing (modified files only)
- 📦 Self-contained tools (one call — complete response)
- 🗂️ Multilingual support (RU, EN, ZH)
- 🦀 Built in Rust — fast and memory-efficient

## Anti-Hallucination Protection

To prevent AI models from making up parameters and tool names:

- ✅ Soft parameter validation
- 🛡️ Input parameter normalization
- 📋 Optimized tool descriptions
- 🏷️ 150+ aliases for tool names
- 🌐 Automatic query language detection
- 🧪 Full-cycle end-to-end tests
- 📐 Tested on models from 3B parameters

## Who This Is For

- **Development teams** — index your entire codebase, and AI assistants can answer questions about architecture, find functions, and explain module connections
- **Developers under NDA** — the entire stack runs locally: OpenSearch, Ollama, MCP server. No data ever leaves to external APIs. Index proprietary code without risk of violating agreements
- **Private projects** — models from 3B parameters run on your hardware. No one sees your code or your queries
- **Technical writers** — store documentation in Markdown files and search it in any language
- **Open-source projects** — give contributors a quick way to understand the code
- **Startups** — lower the entry barrier for new developers without cloud API costs

## What's Next

- [Quick Start](./getting-started.md) — installation, setup, and first run
- [Architecture](./architecture.md) — how everything works under the hood
- [MCP Tools](./mcp-tools.md) — complete reference for all 11 tools
