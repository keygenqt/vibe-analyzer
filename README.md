# Vibe Analyzer

A tool for scanning and analyzing codebases with AST extraction and JSON export.

## Usage

````bash
# Source management
vibe-analyzer source add <path>                  # Add knowledge source
vibe-analyzer source remove --target <path>      # Remove source
vibe-analyzer source list                        # List all sources

# Scanning
vibe-analyzer scan ast                           # All sources (AST only)
vibe-analyzer scan ast --target <name>           # Specific source
vibe-analyzer scan analyze --target <name>       # With LLM enrichment

# Search
vibe-analyzer search ast <query>                 # Search in code structure
vibe-analyzer search ast <query> --target <name> # Search in specific source
vibe-analyzer search ast <query> --limit <n>     # Limit results
vibe-analyzer search hash --target <name>        # Get file hashes (incremental)

# MCP Server
vibe-analyzer serve start                        # Start MCP server
vibe-analyzer serve status                       # Check server status
vibe-analyzer serve stop                         # Stop server

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
````

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
