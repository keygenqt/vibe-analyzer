# Configuration

Vibe Analyzer uses a JSON5 configuration file. JSON5 is an extended version of JSON with support for comments, trailing commas, and other convenient features.

## Location

The configuration file is located at:

```
~/.vibe-analyzer/config.json5
```

The file is created automatically with default settings the first time any CLI command is run.

## Configuration Structure

The configuration consists of these sections: `version`, `log_level`, `opensearch`, `mcp`, `scan`, `analyze`, `export`, `ollama`, and `sources`.

### Full Example with Comments

```json
{
  // Configuration version — do not modify manually
  "version": "0.0.2",

  // Log level: cli, error, warn, info, debug, trace
  "log_level": "cli",

  // OpenSearch connection
  "opensearch": {
    "host": "http://localhost:9200"
  },

  // MCP server
  "mcp": {
    "host": "0.0.0.0",
    "port": 9020,
    "protocol": "latest"
  },

  // AST extraction filters
  "scan": {
    "imports": true,
    "functions": true,
    "variables": true,
    "enums": true,
    "interfaces": true,
    "classes": true,
    "structs": true,
    "comments": true,
    "header_comments": true,
    "headings": true,
    "links": true,
    "code_blocks": true
  },

  // LLM enrichment batch settings
  "analyze": {
    "max_chunk_chars": 4000,
    "max_chunk_files": 3
  },

  // Default export settings
  "export": {
    "format": "json5",
    "output_dir": "/Users/keygenqt/Downloads"
  },

  // Ollama LLM servers — specify multiple for load distribution
  "ollama": [
    {
      "host": "http://localhost:11434",
      "model": "qwen2.5-coder:3b-instruct",
      "timeout_secs": 60,
      "temperature": 0.1,
      "seed": 42,
      "num_ctx": 4096,
      "num_predict": 2048
    },
    {
      "host": "http://192.168.1.10:11434",
      "model": "qwen2.5-coder:3b-instruct",
      "timeout_secs": 60,
      "temperature": 0.1,
      "seed": 42,
      "num_ctx": 4096,
      "num_predict": 2048
    }
  ],

  // Knowledge sources for indexing
  "sources": ["/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer"]
}
```

## Sections in Detail

### version

```json
"version": "0.0.2"
```

Configuration file version. **Do not modify manually** — updated automatically during config migration between versions.

### log_level

```json
"log_level": "cli"
```

Application log level. Supported values: `cli`, `error`, `warn`, `info`, `debug`, `trace`. The `cli` level disables tracing output for clean command-line output.

### opensearch

```json
"opensearch": {
  "host": "http://localhost:9200"
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

### scan

Controls which AST elements are included in the output of `scan ast` and `scan analyze` exports.
Useful for reducing export file size by excluding unneeded element types.

```json
"scan": {
  "imports": true,
  "functions": true,
  "variables": true,
  "enums": true,
  "interfaces": true,
  "classes": true,
  "structs": true,
  "comments": true,
  "header_comments": true,
  "headings": true,
  "links": true,
  "code_blocks": true
}
```

| Parameter         | Type    | Default | Description                          |
| ----------------- | ------- | ------- | ------------------------------------ |
| `imports`         | boolean | `true`  | Include import statements            |
| `functions`       | boolean | `true`  | Include function signatures          |
| `variables`       | boolean | `true`  | Include variable declarations        |
| `enums`           | boolean | `true`  | Include enum declarations            |
| `interfaces`      | boolean | `true`  | Include interface/trait declarations |
| `classes`         | boolean | `true`  | Include class declarations           |
| `structs`         | boolean | `true`  | Include struct declarations          |
| `comments`        | boolean | `true`  | Include doc comments                 |
| `header_comments` | boolean | `true`  | Include file header comments         |
| `headings`        | boolean | `true`  | Include Markdown headings            |
| `links`           | boolean | `true`  | Include Markdown links               |
| `code_blocks`     | boolean | `true`  | Include code blocks                  |

### analyze

```json
"analyze": {
  "max_chunk_chars": 4000,
  "max_chunk_files": 3
}
```

| Parameter         | Type    | Default | Description                                                                                                                 |
| ----------------- | ------- | ------- | --------------------------------------------------------------------------------------------------------------------------- |
| `max_chunk_chars` | integer | `4000`  | Maximum characters per LLM request. Files are grouped into batches until the total size exceeds the limit                   |
| `max_chunk_files` | integer | `3`     | Maximum files per request. Even if the character limit is not reached, no more than this number of files will be in a batch |

### export

```json
"export": {
  "format": "json5",
  "output_dir": "/Users/keygenqt/Downloads"
}
```

| Parameter    | Type   | Default       | Description                                                           |
| ------------ | ------ | ------------- | --------------------------------------------------------------------- |
| `format`     | string | `json5`       | Default export format: `json`, `json5`, `toml`, `toon`, `xml`, `yaml` |
| `output_dir` | string | `~/Downloads` | Default output directory for exported files                           |

### ollama

The `ollama` section is an **array** of Ollama server configurations. One or more hosts can be specified for load distribution.

```json
"ollama": [
  {
    "host": "http://localhost:11434",
    "model": "qwen2.5-coder:3b-instruct",
    "timeout_secs": 60,
    "temperature": 0.1,
    "seed": 42,
    "num_ctx": 4096,
    "num_predict": 2048
  }
]
```

| Parameter      | Type    | Default                     | Description                                                                                                                         |
| -------------- | ------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `host`         | string  | `http://localhost:11434`    | Ollama API endpoint                                                                                                                 |
| `model`        | string  | `qwen2.5-coder:3b-instruct` | Model name for enrichment. Must be pre-loaded via `ollama pull`                                                                     |
| `timeout_secs` | integer | `60`                        | Ollama request timeout in seconds                                                                                                   |
| `temperature`  | float   | `0.1`                       | Generation temperature. `0.0` — maximally deterministic, `1.0` — maximally creative. Low values are recommended for code enrichment |
| `seed`         | integer | `42`                        | Random generator seed. Same seed guarantees reproducible results                                                                    |
| `num_ctx`      | integer | `4096`                      | Model context window size in tokens                                                                                                 |
| `num_predict`  | integer | `2048`                      | Maximum tokens in response                                                                                                          |

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

## Overriding the Configuration File

For testing or custom scenarios, the config file path can be overridden:

```bash
# Use a custom config file
vibe-analyzer --config /custom/path/config.json5 source list
```

The default is `~/.vibe-analyzer/config.json5`.
