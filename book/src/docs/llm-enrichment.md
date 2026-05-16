# LLM Enrichment

After AST parsing, Vibe Analyzer enriches results via Ollama across four analysis modes: Meta, Debt, Errors, and Advice.

## How It Works

```
AST data → static analysis → batching → Ollama request → enrichment results
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

**Prompt.** For Meta, Ollama receives a JSON template to fill. For Debt/Errors/Advice, the model returns a JSON array of issues found.

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

If multiple Ollama hosts are configured, files are distributed via competing consumers:

- All hosts read from a single channel
- The fastest one processes the most
- At the end, per-host statistics are reported

### 4. JSON Repair

LLMs often corrupt JSON: add comments, wrap in markdown blocks, drop quotes. `clean_llm_json` fixes this automatically.

### 5. Retries

If Ollama returns fewer files than were in the batch — up to 5 retries with delay. After 5 failures, an error is returned with recommendations.

## Generation Parameters

Configured per Ollama host:

| Parameter      | Default | Description                        |
| -------------- | ------- | ---------------------------------- |
| `temperature`  | `0.1`   | Low temperature for stable results |
| `seed`         | `42`    | Fixed seed for reproducibility     |
| `num_ctx`      | `4096`  | Context window size in tokens      |
| `num_predict`  | `2048`  | Maximum tokens in response         |
| `timeout_secs` | `60`    | Request timeout in seconds         |

## Model Warm-Up

Before enrichment, for each Ollama host:

1. Availability check
2. Model presence check
3. Empty request to load the model into memory

## Exporting Results

```bash
# AST with LLM enrichment
vibe-analyzer analyze export -m meta,debt

# Code only, JSON5 format
vibe-analyzer analyze export -m errors -t code -f json5
```

Supported export formats: JSON, JSON5, TOML, TOON, XML, YAML.
