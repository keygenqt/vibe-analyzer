# Integrations

Vibe Analyzer provides an MCP server that AI assistants can connect to via the Model Context Protocol. Once connected, the model gains 11 tools for searching code and documentation.

## How It Works from the User's Perspective

The user communicates with the AI assistant in natural language. The model decides which tool to call. Examples from real testing scenarios:

### Code Search

| User Query                                      | Tool                       | What Happens                                                                     |
| ----------------------------------------------- | -------------------------- | -------------------------------------------------------------------------------- |
| "Find `add` functions in the samples project"   | `search_by_code_functions` | Searches for functions with `add` in the signature, returns files and signatures |
| "What classes are in samples?"                  | `search_by_code_classes`   | Returns all classes, structs, interfaces                                         |
| "Show all enums in samples"                     | `search_by_code_variables` | Enums are also searched through this tool                                        |
| "What libraries are used in samples?"           | `search_by_code_imports`   | List of all imports in the project                                               |
| "List files that have the `MAX_VALUE` constant" | `search_by_code_variables` | Search by constant name                                                          |

### File Viewing

| User Query                             | Tool               |
| -------------------------------------- | ------------------ |
| "Show the contents of `src/main.rs`"   | `get_file_content` |
| "Show the structure of `main.py`"      | `get_file_ast`     |
| "What functions are in `src/main.rs`?" | `get_file_ast`     |
| "Open `utils.py`"                      | `get_file_content` |

### Documentation and Knowledge Base Search

| User Query                           | Tool                   |
| ------------------------------------ | ---------------------- |
| "Who is Zizikosh?"                   | `search_documentation` |
| "Tell me about Kukyrbur's abilities" | `search_documentation` |
| "Find Python coding guidelines"      | `search_documentation` |
| "Show the release process"           | `search_documentation` |
| "Find the code review checklist"     | `search_documentation` |

### Project Navigation

| User Query                                 | Tool            |
| ------------------------------------------ | --------------- |
| "What projects are in the database?"       | `show_projects` |
| "Show the tree of the samples project"     | `show_tree`     |
| "How many files are in knowledge?"         | `show_stats`    |
| "Show overall statistics for all projects" | `show_stats`    |

### Administration

| User Query         | Tool         |
| ------------------ | ------------ |
| "Update the index" | `admin_sync` |
| "Reindex projects" | `admin_sync` |

## How to Phrase Queries

The model understands queries in natural language. You don't need to use exact tool names — plain language is enough.

**Good:**

- "Find add functions in the samples project"
- "What classes are in samples?"
- "Show the contents of src/main.rs"
- "Who is Zizikosh?"

**Unnecessary (the model will understand via AliasHandler anyway, but it's better to avoid):**

- "Call search_by_code_functions with query=add"
- "Use the get_file_content tool for path=src/main.rs"

## Important Notes

- **Project names** — you can use the full path or directory name: `"samples"` or `"/path/to/samples"`
- **File paths** — relative to the project root: `"src/main.rs"`, partial matching is supported
- **Result limit** — default 3, maximum 10. If the model requests "all", the limit is automatically raised
- **One call is enough** — the model is trained to respond after a single tool call, no need to ask again

## Connecting to Open WebUI

1. Start the MCP server:

   ```bash
   vibe-analyzer serve start
   ```

2. In Open WebUI settings, add a new MCP server:
   - **URL:** `http://localhost:9020`
   - **Transport:** Streamable HTTP

3. Tools appear automatically

## Connecting to Claude Desktop

Add to the configuration:

```json
{
  "mcpServers": {
    "vibe-analyzer": {
      "url": "http://localhost:9020",
      "transport": "streamable-http"
    }
  }
}
```

## MCP Protocol

Supported versions: `2024-11-05`, `2025-03-26`, `2025-06-18`, `latest`. Configured in the settings:

```json
{
  "mcp": {
    "host": "127.0.0.1",
    "port": 9020,
    "protocol": "latest"
  }
}
```

## Security

- Server without authentication — for trusted networks or localhost
- Default host `127.0.0.1` (local only)
- `0.0.0.0` — for access from Docker containers or other machines
- Server only reads data, `admin_sync` is the only tool that triggers background indexing
