# CLI Reference

Vibe Analyzer provides a command-line interface for managing knowledge sources, analysis, exporting, indexing, and running the MCP server.

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
vibe-analyzer source add /home/user/projects/my-app
vibe-analyzer source remove --target my-app
vibe-analyzer source list
```

### analyze — Analysis and Export

Extract code structure, run LLM enrichment, and export results.

```bash
vibe-analyzer analyze export [options]
```

| Option                  | Description                                                                  |
| ----------------------- | ---------------------------------------------------------------------------- |
| `--target <path>`       | Process a specific source. If not specified — all sources are processed      |
| `-m, --mode <mode>`     | Analysis modes: `ast`, `meta`, `debt`, `errors`, `advice` (comma-separated)  |
| `-t, --type <type>`     | File types to export: `code`, `markdown`, `text`, `binary` (comma-separated) |
| `-f, --format <format>` | Export format: `json` (default), `json5`, `toml`, `toon`, `xml`, `yaml`      |
| `-p, --path <path>`     | Source path for direct file/directory scanning                               |
| `-o, --output <path>`   | Export path. If not specified — file is created in `~/Downloads/`            |

**Examples:**

```bash
# AST only, all sources
vibe-analyzer analyze export

# AST + LLM enrichment for code files only
vibe-analyzer analyze export -m meta,debt -t code

# Errors in markdown documentation only
vibe-analyzer analyze export -m errors -t markdown

# Full analysis with JSON5 export
vibe-analyzer analyze export -m meta,debt,errors,advice -f json5
```

### index — OpenSearch Indexing

Full cycle: AST parsing → LLM enrichment → write to OpenSearch.

```bash
vibe-analyzer analyze index [options]
```

| Option            | Description                                                         |
| ----------------- | ------------------------------------------------------------------- |
| `--target <path>` | Index a specific source. If not specified — all sources             |
| `--force`         | Force full reindexing. Ignores hashes and processes all files again |

**Examples:**

```bash
vibe-analyzer analyze index
vibe-analyzer analyze index --target my-app
vibe-analyzer analyze index --target my-app --force
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

**Example output:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Language                  Files           Lines     AST Objects            Size
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Rust                        194           15146            1631       498.26 KB
Markdown                     33            2884             296       102.64 KB
...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total                       287           24893            2082       755.11 KB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### stats tree

```bash
vibe-analyzer stats tree --target <project> [options]
```

| Option                   | Description                                    |
| ------------------------ | ---------------------------------------------- |
| `--target <path>`        | Project to display the tree for (**required**) |
| `-L`, `--level <number>` | Maximum tree depth (default: 3)                |

### serve — MCP Server

Start, stop, and check MCP server status.

```bash
vibe-analyzer serve <subcommand>
```

| Subcommand | Description                    |
| ---------- | ------------------------------ |
| `start`    | Start MCP server               |
| `stop`     | Stop running server (reserved) |
| `status`   | Show server status (reserved)  |

**Examples:**

```bash
vibe-analyzer serve start
vibe-analyzer serve start --port 9020 --host 127.0.0.1
```

## Export Formats

Six formats available with `--format`:

| Format | Key     | Extension | Description                                               |
| ------ | ------- | --------- | --------------------------------------------------------- |
| JSON   | `json`  | `.json`   | Compact JSON without extra whitespace — minimal file size |
| JSON5  | `json5` | `.json5`  | JSON5 with comments and trailing commas — human-readable  |
| TOML   | `toml`  | `.toml`   | TOML format                                               |
| TOON   | `toon`  | `.toon`   | TOON format — token-efficient output, optimized for LLMs  |
| XML    | `xml`   | `.xml`    | XML with pretty-print formatting                          |
| YAML   | `yaml`  | `.yaml`   | YAML format                                               |
