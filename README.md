# Vibe Analyzer

A tool for scanning and analyzing codebases with AST extraction and JSON export.

## Usage

```bash
# Source management
vibe-analyzer source add /path/to/source
vibe-analyzer source remove --target /path/to/source
vibe-analyzer source list

# Scanning
vibe-analyzer scan ast                          # all sources from config
vibe-analyzer scan ast --target my-project      # specific source
vibe-analyzer scan analyze --target my-project

# Search
vibe-analyzer search ast UserService            # search in code structure
vibe-analyzer search file config                # search in file content
vibe-analyzer search meta authentication        # search in metadata
vibe-analyzer search get abc123def456           # get file content by ID

# MCP Server
vibe-analyzer serve start                       # start MCP server
vibe-analyzer serve status                      # check server status
vibe-analyzer serve stop                        # stop server
```

## Output

JSON file in `~/Downloads/` with the following structure:

```json
{
  "indexed_at": "2026-04-04T21:04:53+03:00",
  "sources": [{
    "source": "/path",
    "stats": { "total_functions": 141, "total_files": 55, ... },
    "files": [...]
  }]
}
```

## Supported Languages

Rust, Python, JavaScript, TypeScript, Java, Go, C#, Kotlin, Swift, Dart, Bash, Batch, ArkTS

## Config

`~/.vibe-analyzer/config.json`

## TODO

- [ ] Orphaned files cleanup (remove deleted files from index)
- [ ] OpenSearch authentication support
- [ ] CLI search commands expansion (meta, file, get, stats)
- [ ] MCP server implementation
- [ ] Watch mode for automatic incremental indexing on file changes
