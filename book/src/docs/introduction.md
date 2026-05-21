# Introduction

## What is Vibe Analyzer

Vibe Analyzer is a codebase analysis and Agentic RAG engine. It extracts structure from source code via AST parsing, enriches it with LLM, runs static analysis for 14 languages, and indexes everything into OpenSearch. AI assistants access knowledge through 11 MCP tools.

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

- 🌳 AST parsing for 14 programming languages
- 🔍 Static analysis: code markers (TODO, FIXME, HACK), warnings (unwrap, panic) across all languages
- 💡 LLM enrichment: technical debt, bugs, refactoring suggestions, file summaries
- 🖥️ Multi-node LLM cluster — distribute enrichment across local and cloud models
- 🌐 Multi-provider support: Ollama, DeepSeek, Qwen
- 📄 Export AST and analysis results to JSON, JSON5, TOML, TOON, XML, YAML
- 📝 Semantic and morphological search across code and documentation
- ⚡ Incremental indexing (modified files only)
- 📦 Self-contained MCP tools (one call — complete response)
- 🗂️ Multilingual support (RU, EN, ZH)
- 🦀 Built in Rust — fast and memory-efficient

## Who This Is For

- **Development teams** — index your codebase, let AI assistants answer architecture questions
- **Developers under NDA** — the entire stack runs locally: OpenSearch, Ollama, MCP server. No data ever leaves to external APIs
- **Private projects** — models from 3B parameters run on your hardware
- **Technical writers** — store documentation in Markdown files and search it in any language
- **Open-source projects** — give contributors a quick way to understand the code

## What's Next

- [Quick Start](./getting-started.md) — installation, setup, and first run
- [Architecture](./architecture.md) — how everything works under the hood
- [MCP Tools](./mcp-tools.md) — complete reference for all 11 tools
