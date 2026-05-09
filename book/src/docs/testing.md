# Testing

Vibe Analyzer uses end-to-end tests powered by the **[vibe-tests](https://crates.io/crates/vibe-tests)** framework to verify MCP tool relevance.

## How It Works

```
engine_config! → engine.test("natural language query") → LLM selects tool → tool returns result → verify
```

The framework:

- Starts the MCP server automatically
- Runs queries against real Ollama models (3B, 7B)
- Verifies the model selected the correct tool
- Saves structured JSON reports with timing, tool calls, and responses

## Test Scenarios

All 11 MCP tools are covered, each with 5 queries in Russian and English:

```
tests/mcp/
├── admin_sync.rs
├── get_file_ast.rs
├── get_file_content.rs
├── search_by_code_classes.rs
├── search_by_code_functions.rs
├── search_by_code_imports.rs
├── search_by_code_variables.rs
├── search_documentation.rs
├── show_projects.rs
├── show_stats.rs
└── show_tree.rs
```

Total: 60 queries across 11 tools × 2 models = 120 tests.

**Example test:**

```rust
#[tokio::test]
async fn test_search_functions_add_ru() {
    let engine = vibe_tests::engine().await;
    let result = engine.test("Найди функции add в проекте 'samples'").await;
    assert!(result.success);
    assert!(result.models.iter().all(|m| m.tool.as_deref() == Some("search_by_code_functions")));
}
```

## Test Infrastructure

- **OpenSearch** — via Docker Compose
- **MCP server** — started automatically on port 9021
- **Fixtures** — test projects `samples` (code) and `knowledge` (docs, legends)
- **Multi-model** — each query tested against multiple Ollama models for robustness

## Running

```bash
# Full E2E tests (require Docker + Ollama)
cargo test --test mcp_test -- --nocapture
```

## Expected Model Behavior

The test verifies that the model:

1. **Called a tool** — the correct tool for the query
2. **Did not call a non-existent tool** — alias resolution works
3. **The tool returned a result** — non-empty response

## Reports

After each run, a structured JSON report is saved with per-query details: model, tool, args, response, duration, and success status.
