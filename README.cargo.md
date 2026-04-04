# Vibe Analyzer

A tool for scanning and analyzing codebases with AST extraction and JSON export.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Project management
vibe-analyzer add /path/to/project
vibe-analyzer del /path/to/project
vibe-analyzer list

# Scanning
vibe-analyzer scan                    # all projects from config
vibe-analyzer scan --path ./src       # specific path
```

## Output

JSON file in `~/Downloads/` with the following structure:

```json
{
  "indexed_at": "2026-04-04T21:04:53+03:00",
  "projects": [{
    "project": "/path",
    "stats": { "total_functions": 141, "total_files": 55 },
    "files": [...]
  }]
}
```

## Supported Languages

Rust, Python, JS/TS, Java, Go, C++, C#, PHP, Ruby, Swift, Kotlin, Dart, SQL, Haskell, Bash, XML, ArkTS

## Config

`~/.vibe-analyzer/config.json`
