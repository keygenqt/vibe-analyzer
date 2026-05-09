# CLI Reference

Vibe Analyzer provides a command-line interface for managing knowledge sources, scanning, exporting, indexing, and running the MCP server.

## General Syntax

```bash
vibe-analyzer [global options] <command> [subcommand] [options]
```

## Global Options

| Option            | Description                                                    |
| ----------------- | -------------------------------------------------------------- |
| `--config <path>` | Path to config file (default: `~/.vibe-analyzer/config.json5`) |
| `--help`          | Show help                                                      |
| `--version`       | Show version                                                   |

## Commands

### source — Source Management

Add, remove, and list knowledge sources.

```bash
vibe-analyzer source <subcommand>
```

| Subcommand               | Description                                                                                       |
| ------------------------ | ------------------------------------------------------------------------------------------------- |
| `add <path>`             | Adds a new directory or file to the sources list. The path is automatically converted to absolute |
| `remove --target <path>` | Removes a source from the configuration. Accepts full path or unique directory name               |
| `list`                   | Shows all added sources with absolute paths                                                       |

**Examples:**

```bash
# Add a project
vibe-analyzer source add /home/user/projects/my-app

# Add a documentation directory
vibe-analyzer source add /home/user/docs

# Remove by full path
vibe-analyzer source remove --target /home/user/projects/my-app

# Remove by directory name (if unique)
vibe-analyzer source remove --target my-app

# List all sources
vibe-analyzer source list
```

**Example `source list` output:**

```
Configured sources:
- /Users/keygenqt/Documents/Gitcode/Projects/vibe-analyzer
- /home/user/projects/my-backend
```

### scan — Scanning and Indexing

Extract code structure via AST parsing, optional LLM enrichment, and OpenSearch indexing.

```bash
vibe-analyzer scan <subcommand>
```

| Subcommand | Description                                                                                                                 |
| ---------- | --------------------------------------------------------------------------------------------------------------------------- |
| `ast`      | AST parsing only. Fast code structure extraction without LLM. Results can be exported to a file                             |
| `analyze`  | Full cycle: AST parsing → LLM enrichment. **Does not index to OpenSearch automatically.** Results can be exported to a file |
| `index`    | OpenSearch indexing. Runs `scan analyze` with incremental updates and writes results to indices                             |

#### scan ast

```bash
vibe-analyzer scan ast [options]
```

| Option                  | Description                                                             |
| ----------------------- | ----------------------------------------------------------------------- |
| `--target <path>`       | Process a specific source. If not specified — all sources are processed |
| `-f, --format <format>` | Export format: `json` (default), `json5`, `toml`, `toon`, `xml`, `yaml` |
| `-p, --path <path>`     | Source path for direct file/directory scanning                          |
| `-o, --output <path>`   | Export path. If not specified — file is created in `~/Downloads/`       |

**Examples:**

```bash
# AST for all sources
vibe-analyzer scan ast

# AST for a specific project
vibe-analyzer scan ast --target my-app

# AST with JSON5 export
vibe-analyzer scan ast --target my-app --format json5

# AST export to a specific file
vibe-analyzer scan ast --target my-app --format json --output /path/to/output.json
```

#### scan analyze

```bash
vibe-analyzer scan analyze [options]
```

| Option                  | Description                                                             |
| ----------------------- | ----------------------------------------------------------------------- |
| `--target <path>`       | Process a specific source. If not specified — all sources are processed |
| `-f, --format <format>` | Export format: `json` (default), `json5`, `toml`, `toon`, `xml`, `yaml` |
| `-p, --path <path>`     | Source path for direct file/directory scanning                          |
| `-o, --output <path>`   | Export path. If not specified — file is created in `~/Downloads/`       |

Before enrichment, it checks:

1. That Ollama hosts are configured
2. That all Ollama servers are reachable (healthcheck)
3. Model warm-up on all servers

If any server is unavailable — the command fails with an error.

**Examples:**

```bash
# Full cycle for all sources
vibe-analyzer scan analyze

# Full cycle for a specific project
vibe-analyzer scan analyze --target my-app

# With export
vibe-analyzer scan analyze --target my-app --format json5
```

#### scan index

```bash
vibe-analyzer scan index [options]
```

| Option            | Description                                                         |
| ----------------- | ------------------------------------------------------------------- |
| `--target <path>` | Index a specific source. If not specified — all sources             |
| `--force`         | Force full reindexing. Ignores hashes and processes all files again |

**What `scan index` does:**

1. Checks OpenSearch availability
2. Cleans up orphaned data (OpenSearch documents no longer in the source)
3. If not `--force` — loads hashes of already indexed files for incremental update
4. Runs `scan_enriches` (AST + LLM), skipping files with unchanged hashes
5. Indexes project metadata (meta)
6. Indexes file contents (files)
7. Indexes file analysis (files_analysis)
8. Prints a report

**Example `scan index` output:**

```
Indexing completed. Files: 287 (took 45.3s)
```

Or, if all files are already indexed:

```
Index is up to date — all files are already in sync
```

**Examples:**

```bash
# Incremental indexing for all sources
vibe-analyzer scan index

# Index a specific project
vibe-analyzer scan index --target my-app

# Force full reindexing
vibe-analyzer scan index --target my-app --force
```

### stats — Statistics

View information and statistics for indexed projects.

```bash
vibe-analyzer stats <subcommand>
```

#### stats info

```bash
vibe-analyzer stats info [options]
```

| Option            | Description                                                             |
| ----------------- | ----------------------------------------------------------------------- |
| `--target <path>` | Show statistics for a specific project. If not specified — all projects |

Requires an **active OpenSearch connection** and indexed data.

**Example output:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Language                  Files           Lines     AST Objects            Size
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Rust                        194           15146            1631       498.26 KB
Markdown                     33            2884             296       102.64 KB
Python                        4             163              23         2.85 KB
...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Other                        45            5776                       137.72 KB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total                       287           24893            2082       755.11 KB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Stats generated in 0.5s
```

**Columns:**

| Column        | Description                                                                                                              |
| ------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `Language`    | Programming language. `Other` — files without AST (images, configs, binaries)                                            |
| `Files`       | Number of files for this language                                                                                        |
| `Lines`       | Total line count                                                                                                         |
| `AST Objects` | Number of extracted AST elements (functions, classes, structs, enums, interfaces, variables, imports, Markdown headings) |
| `Size`        | Total file size                                                                                                          |

#### stats tree

```bash
vibe-analyzer stats tree --target <project> [options]
```

| Option                   | Description                                    |
| ------------------------ | ---------------------------------------------- |
| `--target <path>`        | Project to display the tree for (**required**) |
| `-L`, `--level <number>` | Maximum tree depth (default: 3)                |

Requires an **active OpenSearch connection** and indexed data. The tree is built from indexed paths, not by traversing the filesystem.

**Example output:**

```
vibe-analyzer
|-- Cargo.lock
|-- Cargo.toml
|-- LICENSE
|-- README.md
|-- book
|   |-- book.toml
|   `-- src
|       |-- SUMMARY.md
|       `-- index.html
|-- src
|   |-- main.rs
|   |-- cli
|   |   |-- mod.rs
|   |   |-- scan.rs
|   |   `-- serve.rs
|   `-- utils
|       `-- ...
`-- tests
    `-- ...

36 directories, 74 files
```

Summary at the end of the output with directory count, file count, and build time:

```
Found 36 directories, 74 files in 0.5s
```

### serve — MCP Server

Start, stop, and check MCP server status.

```bash
vibe-analyzer serve <subcommand>
```

#### serve start

```bash
vibe-analyzer serve start [options]
```

| Option                 | Description                                      |
| ---------------------- | ------------------------------------------------ |
| `--host <address>`     | Bind address. Overrides the config value         |
| `--port <port>`        | Server port. Overrides the config value          |
| `--workdir <path>`     | Working directory (default: current directory)   |
| `--protocol <version>` | MCP protocol version. Overrides the config value |

The server runs in foreground mode. Use a system service manager or terminal multiplexer to run in the background.

All parameters are optional — if not specified, values from `config.json5` are used.

**Examples:**

```bash
# Start with config settings
vibe-analyzer serve start

# Start on a specific port
vibe-analyzer serve start --port 9020

# Start on localhost only
vibe-analyzer serve start --host 127.0.0.1 --port 8080
```

#### serve stop and serve status

```
vibe-analyzer serve stop
vibe-analyzer serve status
```

> **Note:** the `stop` and `status` commands are reserved but not yet implemented.

## Export Formats

When using the `--format` option with `scan ast` and `scan analyze`, six formats are available:

| Format | Key     | Extension | Description                                               |
| ------ | ------- | --------- | --------------------------------------------------------- |
| JSON   | `json`  | `.json`   | Compact JSON without extra whitespace — minimal file size |
| JSON5  | `json5` | `.json5`  | JSON5 with comments and trailing commas — human-readable  |
| TOML   | `toml`  | `.toml`   | TOML format                                               |
| TOON   | `toon`  | `.toon`   | TOON format — token-efficient output, optimized for LLMs  |
| XML    | `xml`   | `.xml`    | XML with pretty-print formatting                          |
| YAML   | `yaml`  | `.yaml`   | YAML format                                               |

If the export path is not specified via `--output`, the file is saved to `~/Downloads/` with an auto-generated name.
