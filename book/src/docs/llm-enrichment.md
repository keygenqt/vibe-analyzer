# LLM Enrichment

After AST parsing, Vibe Analyzer enriches results via vibe-cluster across multiple providers in four analysis modes: Meta, Debt, Errors, and Advice.

## How It Works

```
AST data → static analysis → batching → LLM request → enrichment results
```

### 1. Static Analysis

Before LLM processing, code is analyzed programmatically across 14 languages:

- **Markers:** TODO, FIXME, HACK, and 18 other markers
- **Warnings:** unsafe calls (unwrap, panic), swallowed exceptions, debug statements

These are included in the prompt so the model can build on them.

### 2. File Enrichment

**Batching.** Files are grouped into batches limited by `max_chunk_chars` (default 3000) from the `analyze` config section.

The prompt contains the AST structure: functions, classes, structs, and other elements. Which elements to include is controlled by `export.include` in config.

**Analysis modes:**

| Mode     | Description                                                           |
| -------- | --------------------------------------------------------------------- |
| `meta`   | Generate summary and search tags for each file                        |
| `debt`   | Detect technical debt: code duplication, magic numbers, complex logic |
| `errors` | Find bugs: unsafe calls, swallowed errors                             |
| `advice` | Suggest refactoring, naming, and test improvements                    |

**Prompt.** For Meta, the model receives a JSON template to fill. For Debt/Errors/Advice, the model returns a JSON array of issues found.

**Response.** Example for Meta:

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "description": "Main entry point for the CLI application",
      "tags": ["entry", "point", "cli", "argument", "parsing", "rust", "binary"]
    }
  ]
}
```

### 3. Parallel Processing

If multiple cluster nodes are configured, prompts are distributed via vibe-cluster:

- Local models (Ollama) get priority over cloud providers
- Nodes process prompts in parallel via atomic work-stealing
- Configurable `parallel` parameter for multiple workers per provider
- At the end, per-node statistics are reported

### 4. JSON Repair

LLMs often corrupt JSON: add comments, wrap in markdown blocks, drop quotes. `clean_llm_json` fixes this automatically.

### 5. Retries

If a node returns fewer results than expected — automatic retries with delay. After all retries are exhausted, the error is recorded in the result.

## Generation Parameters

Configured per cluster node:

| Parameter      | Default | Description                        |
| -------------- | ------- | ---------------------------------- |
| `temperature`  | `0.1`   | Low temperature for stable results |
| `seed`         | `42`    | Fixed seed for reproducibility     |
| `num_ctx`      | `4096`  | Context window size in tokens      |
| `num_predict`  | `2048`  | Maximum tokens in response         |
| `timeout_secs` | `60`    | Request timeout in seconds         |

## Model Warm-Up

Before enrichment, for each cluster node:

1. Availability check
2. For Ollama nodes: model presence check and warm-up request to load the model into memory

## Exporting Results

```bash
# AST with LLM enrichment
vibe-analyzer analyze export -m meta,debt

# Code only, JSON5 format
vibe-analyzer analyze export -m errors -t code -f json5
```

Supported export formats: JSON, JSON5, TOML, TOON, XML, YAML.
