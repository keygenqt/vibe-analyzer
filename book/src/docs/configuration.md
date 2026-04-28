# Configuration

Vibe Analyzer uses a JSON5 configuration file. JSON5 is an extended version of JSON with support for comments, trailing commas, and other convenient features.

## Location

The configuration file is located at:

```
~/.vibe-analyzer/config.json5
```

The file is created automatically with default settings the first time any CLI command is run.

## Configuration Structure

The configuration consists of four sections: `version`, `opensearch`, `mcp`, `ollama`, and `sources`.

### Full Example with Comments

```json
{
  // Configuration version — do not modify manually
  "version": "0.0.1",

  // OpenSearch connection
  "opensearch": {
    // OpenSearch server URL
    "host": "http://192.168.1.10:9200"
  },

  // MCP server
  "mcp": {
    // Bind address:
    // 0.0.0.0 — accessible from all interfaces (for Docker, remote connections)
    // 127.0.0.1 — local only
    "host": "0.0.0.0",

    // Server port (default: 9020)
    "port": 9020,

    // MCP protocol version:
    // '2024-11-05' — stable
    // '2025-03-26' — improved streaming
    // '2025-06-18' — latest
    // 'latest' — auto-detect
    "protocol": "latest"
  },

  // Ollama LLM servers — specify multiple for load distribution
  "ollama": [
    {
      // Ollama API endpoint
      "host": "http://192.168.1.10:11434",

      // Model for enrichment
      "model": "qwen2.5-coder:3b-instruct",

      // Maximum characters per LLM request
      // Files are grouped into batches until the total size exceeds this limit
      "max_chunk_chars": 4000,

      // Maximum files per LLM request
      "max_chunk_files": 3,

      // Request timeout in seconds
      "timeout_secs": 60,

      // Generation temperature (0.0 — deterministic, 1.0 — creative)
      "temperature": 0.1,

      // Seed for reproducible results (same seed → same output)
      "seed": 42,

      // Model context window size
      "num_ctx": 4096,

      // Maximum tokens in response
      "num_predict": 2048,

      // Which AST elements to include in the prompt
      "ast_imports": false,
      "ast_variables": false,
      "ast_functions": true,
      "ast_enums": true,
      "ast_interfaces": true
    },
    {
      // Second host for load distribution
      "host": "http://localhost:11434",
      "model": "qwen2.5-coder:3b-instruct",
      "max_chunk_chars": 4000,
      "max_chunk_files": 3,
      "timeout_secs": 60,
      "temperature": 0.1,
      "seed": 42,
      "num_ctx": 4096,
      "num_predict": 2048,
      "ast_imports": false,
      "ast_variables": false,
      "ast_functions": true,
      "ast_enums": true,
      "ast_interfaces": true
    }
  ],

  // Knowledge sources for indexing — absolute project paths
  "sources": ["/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer"]
}
```

## Sections in Detail

### version

```json
"version": "0.0.1"
```

Configuration file version. **Do not modify manually** — updated automatically during config migration between versions.

### opensearch

```json
"opensearch": {
  "host": "http://192.168.1.10:9200"
}
```

| Parameter | Type   | Default                 | Description                                                  |
| --------- | ------ | ----------------------- | ------------------------------------------------------------ |
| `host`    | string | `http://localhost:9200` | OpenSearch server URL. Can point to a local or remote server |

### mcp

```json
"mcp": {
  "host": "0.0.0.0",
  "port": 9020,
  "protocol": "latest"
}
```

| Parameter  | Type    | Default     | Description                                                                                               |
| ---------- | ------- | ----------- | --------------------------------------------------------------------------------------------------------- |
| `host`     | string  | `127.0.0.1` | Server bind address. `0.0.0.0` — accessible externally (Docker, remote clients), `127.0.0.1` — local only |
| `port`     | integer | `9020`      | MCP server port                                                                                           |
| `protocol` | string  | `latest`    | MCP protocol version: `2024-11-05`, `2025-03-26`, `2025-06-18`, or `latest`                               |

### ollama

The `ollama` section is an **array** of Ollama server configurations. One or more hosts can be specified for load distribution.

```json
"ollama": [
  {
    "host": "http://192.168.1.10:11434",
    "model": "qwen2.5-coder:3b-instruct",
    "max_chunk_chars": 4000,
    "max_chunk_files": 3,
    "timeout_secs": 60,
    "temperature": 0.1,
    "seed": 42,
    "num_ctx": 4096,
    "num_predict": 2048,
    "ast_imports": false,
    "ast_variables": false,
    "ast_functions": true,
    "ast_enums": true,
    "ast_interfaces": true
  }
]
```

#### Main Parameters

| Parameter | Type   | Default                     | Description                                                     |
| --------- | ------ | --------------------------- | --------------------------------------------------------------- |
| `host`    | string | `http://localhost:11434`    | Ollama API endpoint                                             |
| `model`   | string | `qwen2.5-coder:3b-instruct` | Model name for enrichment. Must be pre-loaded via `ollama pull` |

#### Batch Processing Parameters

| Parameter         | Type    | Default | Description                                                                                                                 |
| ----------------- | ------- | ------- | --------------------------------------------------------------------------------------------------------------------------- |
| `max_chunk_chars` | integer | `4000`  | Maximum characters per LLM request. Files are grouped into batches until the total size exceeds the limit                   |
| `max_chunk_files` | integer | `3`     | Maximum files per request. Even if the character limit is not reached, no more than this number of files will be in a batch |

#### Generation Parameters

| Parameter      | Type    | Default | Description                                                                                                                         |
| -------------- | ------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `timeout_secs` | integer | `60`    | Ollama request timeout in seconds                                                                                                   |
| `temperature`  | float   | `0.1`   | Generation temperature. `0.0` — maximally deterministic, `1.0` — maximally creative. Low values are recommended for code enrichment |
| `seed`         | integer | `42`    | Random generator seed. Same seed guarantees reproducible results                                                                    |
| `num_ctx`      | integer | `4096`  | Model context window size in tokens                                                                                                 |
| `num_predict`  | integer | `2048`  | Maximum tokens in response                                                                                                          |

#### AST Element Filters

Determines which AST elements are included in the LLM prompt. Disabling unnecessary elements reduces request size and speeds up processing.

| Parameter        | Type    | Default | Description                      |
| ---------------- | ------- | ------- | -------------------------------- |
| `ast_imports`    | boolean | `false` | Include imports in the prompt    |
| `ast_variables`  | boolean | `false` | Include variables in the prompt  |
| `ast_functions`  | boolean | `true`  | Include functions in the prompt  |
| `ast_enums`      | boolean | `true`  | Include enums in the prompt      |
| `ast_interfaces` | boolean | `true`  | Include interfaces in the prompt |

### sources

```json
"sources": [
  "/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer",
  "/home/user/projects/my-backend",
  "/home/user/docs/architecture"
]
```

| Parameter | Type             | Default | Description                                                                                                     |
| --------- | ---------------- | ------- | --------------------------------------------------------------------------------------------------------------- |
| `sources` | array of strings | `[]`    | List of absolute paths to knowledge sources. Each source can be a code project, a documentation folder, or both |

Managing sources is easier via CLI rather than editing the config manually:

```bash
# Add a source
vibe-analyzer source add /path/to/project

# Remove a source
vibe-analyzer source remove --target /path/to/project

# List all sources
vibe-analyzer source list
```

## Ollama Clustering

When multiple hosts are specified in the `ollama` section, Vibe Analyzer distributes files via competing consumers:

- All workers read from a single shared channel
- The fastest worker takes the next file
- This maximizes utilization of all hosts
- If any host fails, all workers stop
- At the end, per-host statistics are reported

This approach allows:

- **Faster enrichment** through parallel processing on multiple GPUs/servers
- **Scaling** by adding more hosts to the configuration

## Configuration Validation

Vibe Analyzer validates the configuration at startup and applies safe defaults if parameters are missing or invalid:

- `max_chunk_chars` → minimum 1000, maximum 100000
- `limit` in search queries → always in the 1–10 range
- Invalid paths → normalized to absolute form
- Missing sections → created with default values

## Configuration Migration

When updating Vibe Analyzer, the configuration may automatically migrate to a new format. The configuration version (`version`) tracks the current format and applies migrations when necessary.

## Overriding the Configuration Directory

For testing or custom scenarios, the configuration directory can be overridden:

```bash
# Set a custom directory
vibe-analyzer --config-dir /custom/path source list
```

The default is `~/.vibe-analyzer/`.

## Dev Section

An optional section for debugging. Usually absent in production config — added only when needed:

```json
{
  "dev": {
    "log_level": "trace",
    "spdx_data_path": "tests/mcp/fixtures/config/spdx"
  }
}
```

| Parameter        | Type   | Default                 | Description                                                                             |
| ---------------- | ------ | ----------------------- | --------------------------------------------------------------------------------------- |
| `log_level`      | string | `default`               | Log level: `default`, `trace`, `debug`, `info`, `warn`, `error`. `default` means `info` |
| `spdx_data_path` | string | `~/.vibe-analyzer/spdx` | Path to SPDX data for license detection. Downloaded automatically on first run          |
