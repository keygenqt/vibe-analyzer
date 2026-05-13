# Recipes

Practical examples of using Vibe Analyzer through an AI assistant. Based on the test fixtures: `samples` (code) and `knowledge` (documentation).

## Exploring an Unfamiliar Project

**Goal:** you're seeing a project for the first time and want to quickly understand its structure.

```
You: What projects are available?
AI: [show_projects]
- knowledge — guidelines and processes for code writing
- samples — sample code in Rust and Python

You: Show the tree of samples
AI: [show_tree]
samples/
├── python_sample/
│   ├── main.py
│   ├── models.py
│   └── utils.py
├── rust_lib/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── rust_sample/
    ├── Cargo.toml
    └── src/
        └── main.rs

You: Show statistics for knowledge
AI: [show_stats]
Markdown    15 files    663 lines    112 AST objects

You: What functions are in utils.py?
AI: [get_file_ast for utils.py]
- add(a: int, b: int) -> int — Returns sum of two numbers
- multiply(a: int, b: int) -> int — Returns product of two numbers
- divide(a: int, b: int) -> float — Divides two numbers safely
```

## Finding Dependencies

**Goal:** find out what libraries are used in a project.

```
You: What libraries are used in samples?
AI: [search_by_code_imports]
- thiserror::Error (Rust)
- super::* (Rust)
- dataclasses (Python)
- typing (Python)
- utils, models (Python)
```

## Finding an Implementation

**Goal:** find where a specific function is implemented.

```
You: Find the multiply function
AI: [search_by_code_functions with query="multiply"]
Found in 2 files:
- python_sample/utils.py: def multiply(a: int, b: int) -> int
- rust_lib/src/lib.rs: pub fn multiply(a: i32, b: i32) -> i32
```

## Documentation Search

**Goal:** find rules or processes.

```
You: Find Python coding guidelines
AI: [search_documentation with query="Python coding rules"]
Found in knowledge/guidelines/code/python.md:
  # Python Coding Guidelines
  Follow PEP 8. Use snake_case, type annotations, docstrings.

You: What is the release process?
AI: [search_documentation with query="release process"]
Found in knowledge/guidelines/process/release.md:
  # Release Process
  1. All milestone tasks closed
  2. Create a release branch from develop
  3. Update version in Cargo.toml/package.json
  4. Merge into main
  5. Notify the team
```

## Knowledge Base Search

**Goal:** find information about project characters or legends.

```
You: Who is Zizikosh?
AI: [search_documentation with query="Zizikosh"]
Found in knowledge/legends/zizikosh_blogger.md:
  # Zizikosh - Blogger
  Video blogger, unpacks packages from AliExpress.
  150K subscribers.

You: What is Bakobur's dream?
AI: [search_documentation with query="Bakobur's dream"]
Found in knowledge/legends/bakobur_homeless.md:
  Dream: Warm socks and a doghouse for Sharik
```

## Inspecting a Specific File

**Goal:** view the contents or structure of a specific file.

```
You: Show the contents of Cargo.toml
AI: [get_file_content]
[package]
name = "rust_lib"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
thiserror = "1.0"

You: What structs are in lib.rs?
AI: [get_file_ast]
- struct User { name, age }
- struct Calculator { value }
- enum MathError { DivisionByZero, OutOfRange }
- enum Operation { Add, Multiply, Divide }
```

## Incremental Update

```
You: Update the index
AI: [admin_sync]
Indexing started. Projects are updating.
```

## Tips

- **Start broad** — `show_projects` → `show_tree` → `show_stats`
- **Refine with search** — `search_by_code_functions`, `search_documentation`
- **Inspect details** — `get_file_content`, `get_file_ast`
- **Update the index after changes** — `admin_sync`
- **Use natural language** — the model will choose the right tool automatically
