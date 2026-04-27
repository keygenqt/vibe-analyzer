## Chapter 1. Introduction

### What is Vibe Analyzer

Vibe Analyzer is an **Agentic RAG engine** for code and knowledge bases. It extracts structure from your source code, enriches it with LLM-generated descriptions, and indexes everything into OpenSearch. AI assistants then search your codebase through **11 MCP tools** — not by reading documents, but by calling tools that return structured results.

### The Problem with Traditional RAG

Standard RAG (Retrieval-Augmented Generation) works like this:

```
User asks → Find relevant documents → Stuff them into prompt → Model answers
```

The problem: **larger context = more VRAM = higher cost.** Every found document bloats the prompt, and you pay for every token.

### How Agentic RAG is Different

Vibe Analyzer flips the model:

```
User asks → Model decides which tool to call → Tool returns structured data → Model answers
```

Instead of stuffing documents into context, the model **chooses** what information it needs and **calls a tool** to get it. The result is compact, precise, and cheap:

- **Minimal context** — model receives only what the tool returns, not entire documents
- **No embeddings** — keyword and AST search via OpenSearch, no vector databases
- **Single call = complete answer** — one tool call retrieves exactly what's needed
- **Scales to any project size** — context stays small whether your codebase has 100 files or 100,000

### How It Works

Vibe Analyzer processes code in three stages:

```
┌─────────────┐     ┌──────────────┐     ┌──────────────┐
│   Scanner   │ ──▶ │   Analyzer   │ ──▶ │  OpenSearch  │
│  (AST only) │     │ (LLM enrich) │     │   (Search)   │
└─────────────┘     └──────────────┘     └──────────────┘
```

1. **Scanner** — parses source code with tree-sitter, extracts functions, classes, imports, variables, structs, enums, interfaces, and documentation (headings, links, code blocks for Markdown)

2. **Analyzer** — sends AST to Ollama LLM, which generates human-readable descriptions and multilingual search tags (EN/RU/CN) for every code element

3. **OpenSearch** — indexes everything into three separate indices: metadata, raw files, and enriched AST. MCP server exposes this data through tools

### Key Features

| Feature                  | Description                                                                                                         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| **AST Parsing**          | 14 languages: Rust, Python, JavaScript, TypeScript, Java, Go, C#, Kotlin, Swift, Dart, Bash, Batch, ArkTS, Markdown |
| **LLM Enrichment**       | Descriptions and search tags for every function, class, and file                                                    |
| **Incremental Indexing** | Only re-indexes changed files (content hashing)                                                                     |
| **Multilingual Search**  | English, Russian, Chinese — tags generated automatically                                                            |
| **Export Formats**       | JSON, JSON5, TOON (token-optimized), XML                                                                            |
| **11 MCP Tools**         | Full agency for AI models — they decide what to find and how                                                        |
| **Anti-Hallucination**   | 150+ tool name aliases, parameter normalization, language detection                                                 |
| **Built in Rust**        | Fast, memory-efficient, single binary                                                                               |

### Architecture

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ AI Assistant │ ──▶ │  MCP Server  │ ──▶ │  OpenSearch  │
│     (Claude) │     │  (11 tools)  │     │  (3 indexes) │
└──────────────┘     └──────────────┘     └───────▲──────┘
                                                  │
                         ┌────────────────────────┴──────┐
                         │            Vibe CLI           │
                         │ source → scan → analyze → idx │
                         └───────────────────────────────┘
```

### Who is This For

- **Development teams** — index your entire codebase, let AI assistants answer questions about architecture, find functions, or explain how modules connect
- **Open-source projects** — provide AI-readable documentation alongside traditional docs
- **Knowledge managers** — index guidelines, rules, and project lore as searchable knowledge bases
- **AI power users** — give models agency over your code without bloating context windows

### What's Next

- [Chapter 2: Getting Started](./getting-started.md) — install, configure, run your first search
- [Chapter 5: MCP Tools](./mcp-tools.md) — complete tool reference with examples
