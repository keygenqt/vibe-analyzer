# Testing

Vibe Analyzer uses two types of tests: unit tests for parsers and end-to-end tests for MCP tools.

## Parser Unit Tests

Each of the 13 languages has a test that verifies AST parsing correctness using **snapshot testing**:

```
Source file → parser → AST → comparison with reference JSON
```

**Example test (Rust):**

```rust
#[test]
fn test_rust_parser() {
    let code = fs::read_to_string("tests/parsers/fixtures/rust/sample.rs").unwrap();
    let json = fs::read_to_string("tests/parsers/fixtures/rust/sample.json").unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();

    let ast = parse_ast(&code, "rs").unwrap().unwrap();
    let actual = serde_json::to_value(&ast).unwrap();

    assert_eq!(actual, expected);
}
```

**Fixture structure:**

```
tests/parsers/fixtures/
├── rust/
│   ├── sample.rs      ← source code
│   └── sample.json    ← expected AST
├── python/
│   ├── sample.py
│   └── sample.json
├── markdown/
│   ├── sample.md
│   └── sample.json
└── ... (a pair of files per language)
```

**All parser tests:**

| Test                     | File                 | Language                                 |
| ------------------------ | -------------------- | ---------------------------------------- |
| `test_rust_parser`       | `rust_test.rs`       | Rust (3 tests: sample, sample2, sample3) |
| `test_python_parser`     | `python_test.rs`     | Python                                   |
| `test_javascript_parser` | `javascript_test.rs` | JavaScript                               |
| `test_typescript_parser` | `typescript_test.rs` | TypeScript                               |
| `test_java_parser`       | `java_test.rs`       | Java                                     |
| `test_go_parser`         | `go_test.rs`         | Go                                       |
| `test_csharp_parser`     | `csharp_test.rs`     | C#                                       |
| `test_kotlin_parser`     | `kotlin_test.rs`     | Kotlin                                   |
| `test_swift_parser`      | `swift_test.rs`      | Swift                                    |
| `test_dart_parser`       | `dart_test.rs`       | Dart                                     |
| `test_bash_parser`       | `bash_test.rs`       | Bash                                     |
| `test_batch_parser`      | `batch_test.rs`      | Batch                                    |
| `test_arkts_parser`      | `test_arkts.rs`      | ArkTS                                    |
| `test_python_parser`     | `markdown_test.rs`   | Markdown                                 |

Run:

```bash
cargo test --test parsers_test
```

## End-to-End MCP Tool Tests

E2E tests verify the full cycle: an AI model receives a query, selects a tool, calls it, and returns a response.

### How It Works

```
Scenario (JSON) → Ollama model → MCP tool call → result verification
```

**Two-turn dialog:**

1. **Turn 1 (with tools):** the model receives a query and **must** call exactly one tool
2. **Turn 2 (without tools):** the model receives the tool result and **must** provide a final text response

If the model calls a second tool instead of responding — it's an error.

### Test Scenarios

Scenarios are stored in JSON files:

```
tests/mcp/fixtures/scenarios/
├── admin_sync.json
├── get_file_ast.json
├── get_file_content.json
├── search_by_code_classes.json
├── search_by_code_functions.json
├── search_by_code_imports.json
├── search_by_code_variables.json
├── search_documentation.json
├── show_projects.json
├── show_stats.json
└── show_tree.json
```

**Example scenario (`search_by_code_functions.json`):**

```json
{
  "tool": "search_by_code_functions",
  "queries": [
    "Find add functions in the 'samples' project",
    "What methods are in 'samples'",
    "Show all main functions in 'samples'",
    "Find calculate functions in 'samples'",
    "List files that have the multiply function"
  ]
}
```

Each scenario contains 5 queries in Russian and English — simple, one-sentence, without specifying the exact tool name.

### Models for Testing

```rust
const MODELS: &[&str] = &[
    "qwen2.5-coder:3b-instruct",
    "qwen2.5-coder:7b-instruct",
    "qwen2.5-coder:14b-instruct",
];
```

By default, tests run on `qwen2.5-coder:3b-instruct` — the smallest model that should work correctly.

### Extracting JSON from Model Responses

The model may return a response in different formats. `extract_json` handles all variants:

| Response Format         | Handling                                              |
| ----------------------- | ----------------------------------------------------- |
| ` ```json { ... } ``` ` | Extracted from the markdown block                     |
| ` ``` { ... } ``` `     | Extracted from the block without a language specifier |
| `{ ... }`               | Used as-is                                            |

### Parsing Tool Calls

`parse_tool_call` looks for the tool name in several JSON fields (models name them differently):

```rust
let name = parsed
    .get("name")       // standard
    .or_else(|| parsed.get("function"))  // OpenAI-style
    .or_else(|| parsed.get("tool"))      // alternative
    .or_else(|| parsed.get("method"))    // another variant
    .or_else(|| parsed.get("call"));     // and another
```

### Test Infrastructure

A custom framework was developed for E2E tests that automatically sets up the entire environment:

- **OpenSearch** — via Docker Compose with fixtures from `tests/mcp/fixtures/opensearch/docker-compose.yml`
- **MCP server** — started automatically on port 9021
- **Fixtures** — test projects `samples` and `knowledge` with legendary characters
- **Ollama** — must be running beforehand with the required model

The framework manages the entire lifecycle: starting services, indexing fixtures, running scenarios, saving reports, and stopping the environment on completion.

### Reports

After each query, an intermediate report is saved; after each scenario, a final one:

```json
{
  "test_name": "search_by_code_functions",
  "model": "qwen2.5-coder:3b-instruct",
  "timestamp": "2026-04-28T12:00:00Z",
  "queries": [
    {
      "query": "Find add functions in the 'samples' project",
      "tool_calls": [
        {
          "name": "search_by_code_functions",
          "args": "{\"query\":\"add\",\"target\":\"samples\"}",
          "result": "[{...}]"
        }
      ],
      "response": "Found function add in file src/lib.rs...",
      "duration_ms": 1234
    }
  ],
  "summary": {
    "total_queries": 5,
    "successful_tool_calls": 5,
    "total_duration_ms": 6170,
    "avg_response_time_ms": 1234
  }
}
```

### Running

```bash
# Parser unit tests only (fast)
cargo test --test parsers_test

# Full E2E tests (require Docker + Ollama)
cargo test --test mcp_test -- --ignored --nocapture
```

### Logging

Tests write a structured log to `tests/reports/<timestamp>/mcp_test.log` and simultaneously output to the terminal. Output is filtered by level: INFO shows progress, DEBUG shows model responses, TRACE shows everything including raw docker and MCP server output.

### Expected Model Behavior

The test verifies that the model:

1. **Called a tool on the first turn** — if not, error `Model did not call a tool`
2. **Did not call a non-existent tool** — if it did, error `TOOL_NOT_FOUND`
3. **The tool returned a non-null result** — if null, error `tool returned null`
4. **Provided a text response on the second turn** — if it called another tool, error `Model called second tool`
