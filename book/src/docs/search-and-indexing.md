# Search and Indexing

Vibe Analyzer stores all data in OpenSearch and uses multilingual analyzers for search.

## Three Indices

Three indices are created for each project:

| Index                        | Purpose  | Contents                                                                    |
| ---------------------------- | -------- | --------------------------------------------------------------------------- |
| `vibe_meta`                  | Metadata | 1 document per project: summary, license, README, statistics                |
| `vibe_files_{hash}`          | Content  | One document per file: full contents (not indexed for search, `store` only) |
| `vibe_files_analysis_{hash}` | Search   | One document per text file: AST, description, tags                          |

## Multilingual Search

OpenSearch is configured with three analyzers:

- `russian_analyzer` (type `russian`) — stemming for Russian
- `english_analyzer` (type `english`) — stemming for English
- `chinese_analyzer` (type `chinese`) — segmentation for Chinese

Each text field in `vibe_files_analysis` has three sub-fields — one per analyzer. This allows searching for "функции", "functions", and "函数" with correct morphology for each language.

## Search Mechanics

### Documentation Search (`search_documentation`)

The most complex query. Algorithm:

1. **Script detection in the query** — Cyrillic, Latin, CJK
2. **Word extraction** (longer than 2 characters)
3. **Wildcard search on headings** with 10.0 boost + stemming for long words
4. **Language-specific match queries** — for each detected script, a separate query to the corresponding sub-field with fuzziness
5. **Boost for knowledge documents** — if the frontmatter contains `knowledge: true`, the document gets a 5.0 boost

Ranking priority:

- Headings (`headings.title`) — 10.0 boost
- Preview (`headings.preview`) — 2.0 boost
- Links (`links.text`) — 2.0 boost
- Tags (`tags`) — 1.0 boost

### Code Search

Each search type has its own strategy:

- **Imports** — `wildcard` on the `imports` keyword field + tags
- **Functions** — `match_phrase_prefix` on signatures + `match` on comments (nested queries)
- **Classes/structs/interfaces** — three nested queries in `should` with `minimum_should_match: 1`
- **Variables/enums** — `match_phrase_prefix` on signatures + `match` on comments (nested queries)

All code searches use `fuzziness: AUTO` for fuzzy matching and boost tags higher than specific fields.

## Incremental Indexing

Vibe Analyzer doesn't re-index files unnecessarily:

1. **Fetching hashes** from OpenSearch via Scroll API — `GET /{index}/_search?scroll=1m`
2. **Comparison** — a BLAKE3 hash is computed for each file and compared against the indexed one
3. **Skipping unchanged** — files with matching hashes are not processed

If the `--force` flag is passed, hashes are ignored — all files are indexed.

## Bulk Indexing

All documents are written to OpenSearch in batches via the Bulk API in NDJSON format:

```
{"index": {"_index": "vibe_files_xxx", "_id": "src/main.rs"}}
{"root": "/project", "path": "src/main.rs", "content": "..."}
{"index": {"_index": "vibe_files_xxx", "_id": "src/lib.rs"}}
{"root": "/project", "path": "src/lib.rs", "content": "..."}
```

The document ID is the file path (`path`). This ensures that re-indexing updates the existing document rather than creating a duplicate.

## Orphaned Data Cleanup

`cleanup` runs automatically during indexing:

1. **Index removal** for deleted projects
2. **Document removal** for files no longer on disk (comparing paths in the index and on the filesystem)
3. **Meta-document removal** for projects removed from the configuration

## Project Statistics

`show_stats_search` collects aggregated statistics across all indexed files via the Scroll API. This enables:

- **Project reports** — language breakdown, file count, lines, AST objects
- **Data presence checks** — if statistics are empty, indexing hasn't been performed or the project hasn't been added
- **Codebase size estimation** — total size, text and binary file counts

Aggregation runs across all documents from `files_analysis`:

- Language grouping (via `get_language_name`)
- AST object counting: sum of functions, classes, structs, enums, interfaces, variables, imports, headings, links, code blocks
- `Other` — files without a detectable language
- Languages sorted by lines of code descending
