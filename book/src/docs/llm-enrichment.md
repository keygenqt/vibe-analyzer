# LLM Enrichment

After AST parsing, Vibe Analyzer can enrich results via Ollama: adding a description and search tags to each file, and a brief summary to the project based on the README.

## How It Works

```
AST data → batching → Ollama request → description + tags for each file
```

### 1. Project Summarization

The README (if present) is sent to Ollama with the prompt "write 2-3 sentences about the project". The result is saved in `summary`.

### 2. File Enrichment

**Batching.** Files are grouped into batches based on two config limits:

- `max_chunk_chars` — maximum characters per request (default 4000)
- `max_chunk_files` — maximum files per request (default 3)

The prompt does not contain the files themselves, but their AST: functions, classes, structs, and other elements. Which elements to include is controlled by the flags `ast_imports`, `ast_variables`, `ast_functions`, `ast_enums`, `ast_interfaces`.

**Prompt.** Ollama receives a JSON template:

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "description": "FILL_DESCRIPTION",
      "tags": ["TAG1", "TAG2", "TAG3"]
    }
  ]
}
```

The model must fill in `description` and `tags` while preserving the structure. The prompt strictly requires: don't skip files, don't change paths, copy the JSON as-is.

**Response.** Ollama returns the completed JSON:

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "description": "Main entry point for the CLI application",
      "tags": ["entry-point", "cli", "argument-parsing"]
    }
  ]
}
```

### 3. Parallel Processing

If multiple Ollama hosts are configured, files are distributed among them:

- All hosts read from a single channel
- The fastest one processes the most
- If any host errors — all stop (`error_flag`)
- At the end, per-host statistics are reported: how many files each processed

### 4. JSON Repair

LLMs often corrupt JSON: add comments, wrap in markdown blocks, drop quotes. `clean_llm_json` fixes this:

- Extracts JSON from ``` blocks
- Adds missing key quotes
- Removes trailing commas
- Balances unclosed braces

### 5. Retries

If Ollama returns fewer files than were in the batch — up to 5 retries with delay. If after 5 attempts it still doesn't match — an error with recommendations to reduce `max_chunk_chars` or switch models.

## Generation Parameters

The following are passed from config to the Ollama request:

- `temperature: 0.1` — low temperature for stable results
- `seed: 42` — fixed seed for reproducibility
- `num_ctx: 4096` — context window size
- `num_predict: 2048` — maximum tokens in response
- `timeout_secs: 60` — request timeout

## Model Warm-Up

Before enrichment begins, for each Ollama host:

1. Availability check (`GET /`)
2. Model presence check (`GET /api/tags`)
3. Empty request to load the model into memory (`POST /api/generate` with empty prompt)

## Enrichment Result

After processing, each file receives `description` and `tags` from the LLM, and the project receives a `summary` based on the README.

### Exporting Results

AST parsing and LLM enrichment results can be exported to a file for analysis, debugging, or use in other tools:

```bash
# AST with LLM enrichment
vibe-analyzer scan analyze --target my-app

# With format and path specified
vibe-analyzer scan ast --target my-app --format json5 --output analysis.json5
```
