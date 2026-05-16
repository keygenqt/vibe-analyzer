# Configuration

Vibe Analyzer uses a JSON5 configuration file. JSON5 is an extended version of JSON with support for comments, trailing commas, and other convenient features.

## Location

```
~/.vibe-analyzer/config.json5
```

The file is created automatically with default settings the first time any CLI command is run.

## Configuration Structure

The configuration consists of these sections: `version`, `log_level`, `opensearch`, `mcp`, `analyze`, `export`, `ollama`, and `sources`.

### Full Example with Comments

```json
{
  // Configuration version (do not modify)
  "version": "0.0.3",

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

  // Analysis settings
  "analyze": {
    "max_chunk_chars": 3000,
    "include": {
      "meta": false,
      "debt": false,
      "advice": false,
      "errors": false
    }
  },

  // Default export settings
  "export": {
    "format": "json",
    "output_dir": "/Users/keygenqt/Downloads",
    "types": {
      "code": true,
      "markdown": true,
      "text": true,
      "binary": true
    },
    "include": {
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
      "code_blocks": true,
      "markers": false,
      "warnings": false,
      "body": false
    }
  },

  // Ollama LLM servers
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
  ],

  // Knowledge sources for indexing
  "sources": ["/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer"]
}
```

## Sections in Detail

### version

```json
version: '0.0.3'
```

Configuration file version. **Do not modify manually** — updated automatically during config migration between versions.

### log_level

```json
log_level: 'cli'
```

Application log level. Supported values: `cli`, `error`, `warn`, `info`, `debug`, `trace`. The `cli` level disables tracing output for clean command-line output.

### opensearch

```json
opensearch: {
  "host": "http://localhost:9200",
}
```

| Parameter | Type   | Default                 | Description                                                  |
| --------- | ------ | ----------------------- | ------------------------------------------------------------ |
| `host`    | string | `http://localhost:9200` | OpenSearch server URL. Can point to a local or remote server |

### mcp

```json
mcp: {
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

### analyze

```json
analyze: {
  "max_chunk_chars": 3000,
  "include": {
    "meta": false,
    "debt": false,
    "advice": false,
    "errors": false
  }
}
```

| Parameter         | Type    | Default | Description                              |
| ----------------- | ------- | ------- | ---------------------------------------- |
| `max_chunk_chars` | integer | `3000`  | Maximum characters per LLM request batch |

**Enrichment features:**

| Feature  | Description                                                  |
| -------- | ------------------------------------------------------------ |
| `meta`   | Generate summary and search tags for the file                |
| `debt`   | Detect technical debt: TODO markers, magic numbers           |
| `advice` | Suggest refactoring, naming, and test improvements           |
| `errors` | Find bugs: unsafe calls, swallowed errors, spelling mistakes |

When all features are disabled, only AST data is returned.

### export

```json
export: {
  "format": "json5",
  "output_dir": "/Users/keygenqt/Downloads",
  "types": {
    "code": true,
    "markdown": true,
    "text": true,
    "binary": true
  },
  "include": {
    "imports": true,
    "functions": true,
    "variables": true,
    // ... (see full example above)
    "markers": false,
    "warnings": false,
    "body": false
  }
}
```

| Parameter    | Type   | Default       | Description                                                           |
| ------------ | ------ | ------------- | --------------------------------------------------------------------- |
| `format`     | string | `json5`       | Default export format: `json`, `json5`, `toml`, `toon`, `xml`, `yaml` |
| `output_dir` | string | `~/Downloads` | Default output directory for exported files                           |

**File types (`types`):**

| Type       | Description                             |
| ---------- | --------------------------------------- |
| `code`     | Source code files (Rust, Python, etc.)  |
| `markdown` | Markdown documentation files            |
| `text`     | Text files without AST parser (configs) |
| `binary`   | Binary files (images, archives)         |

**AST elements (`include`):**

| Element           | Description                                 |
| ----------------- | ------------------------------------------- |
| `imports`         | Import statements                           |
| `functions`       | Function signatures                         |
| `variables`       | Variable declarations                       |
| `enums`           | Enum declarations                           |
| `interfaces`      | Interface/trait declarations                |
| `classes`         | Class declarations                          |
| `structs`         | Struct declarations                         |
| `comments`        | Documentation comments                      |
| `header_comments` | File header comments                        |
| `headings`        | Markdown headings                           |
| `links`           | Markdown links                              |
| `code_blocks`     | Markdown code blocks                        |
| `markers`         | Static analysis markers (TODO, FIXME, HACK) |
| `warnings`        | Static analysis warnings (unwrap, panic)    |
| `body`            | Body content (functions, classes, headings) |

### ollama

```json
ollama: [
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

| Parameter      | Type    | Default                     | Description                                                  |
| -------------- | ------- | --------------------------- | ------------------------------------------------------------ |
| `host`         | string  | `http://localhost:11434`    | Ollama API endpoint                                          |
| `model`        | string  | `qwen2.5-coder:3b-instruct` | Model name. Must be pre-loaded via `ollama pull`             |
| `timeout_secs` | integer | `60`                        | Request timeout in seconds                                   |
| `temperature`  | float   | `0.1`                       | Generation temperature (0.0 — deterministic, 1.0 — creative) |
| `seed`         | integer | `42`                        | Random seed for reproducible results                         |
| `num_ctx`      | integer | `4096`                      | Context window size in tokens                                |
| `num_predict`  | integer | `2048`                      | Maximum tokens in response                                   |

Multiple hosts can be specified for load distribution via competing consumers.

### sources

```json
sources: [
  "/Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer",
]
```

| Parameter | Type             | Default | Description                       |
| --------- | ---------------- | ------- | --------------------------------- |
| `sources` | array of strings | `[]`    | List of absolute paths to sources |

Managing sources via CLI:

```bash
vibe-analyzer source add /path/to/project
vibe-analyzer source remove --target /path/to/project
vibe-analyzer source list
```

## Configuration Validation

Vibe Analyzer validates the configuration at startup and applies safe defaults if parameters are missing or invalid:

- `max_chunk_chars` → minimum 1000, maximum 100000
- Invalid paths → normalized to absolute form
- Missing sections → created with default values

## Overriding the Configuration File

```bash
vibe-analyzer --config /custom/path/config.json5 source list
```

Default is `~/.vibe-analyzer/config.json5`.
