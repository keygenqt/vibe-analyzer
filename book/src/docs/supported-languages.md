# Supported Languages

Vibe Analyzer supports AST parsing and LLM enrichment for 13 programming languages and file formats.

## Full List

| Language   | Extensions | AST | Enrichment | Parser             |
| ---------- | ---------- | --- | ---------- | ------------------ |
| Rust       | `.rs`      | ✅  | ✅         | `RustParser`       |
| Python     | `.py`      | ✅  | ✅         | `PythonParser`     |
| JavaScript | `.js`      | ✅  | ✅         | `JavaScriptParser` |
| TypeScript | `.ts`      | ✅  | ✅         | `TypeScriptParser` |
| Java       | `.java`    | ✅  | ✅         | `JavaParser`       |
| Go         | `.go`      | ✅  | ✅         | `GoParser`         |
| C#         | `.cs`      | ✅  | ✅         | `CSharpParser`     |
| Kotlin     | `.kt`      | ✅  | ✅         | `KotlinParser`     |
| Swift      | `.swift`   | ✅  | ✅         | `SwiftParser`      |
| Dart       | `.dart`    | ✅  | ✅         | `DartParser`       |
| Bash       | `.sh`      | ✅  | ✅         | `BashParser`       |
| Batch      | `.bat`     | ✅  | ✅         | `BatchParser`      |
| ArkTS      | `.ets`     | ✅  | ✅         | `ArkTsParser`      |
| Markdown   | `.md`      | ✅  | ✅         | `MarkdownParser`   |

> **Note:** Markdown is a special case. Headings, links, code blocks, and frontmatter metadata are extracted instead of programmatic constructs.
> This allows Markdown files to be used as a knowledge base: documentation, guidelines, standards, project legends. They are searchable via `search_documentation` and `search_knowledge`.

## Extracted Element Categories

### For All Programming Languages

| Element           | Description                          | Example (Rust)                           |
| ----------------- | ------------------------------------ | ---------------------------------------- |
| `header_comments` | Module comment — file purpose        | `"Application configuration management"` |
| `functions`       | Functions and methods                | `fn add(a: i32, b: i32) -> i32`          |
| `classes`         | Classes                              | `class User`                             |
| `structs`         | Structs and records                  | `struct Config`                          |
| `enums`           | Enums                                | `enum Color`                             |
| `interfaces`      | Interfaces, traits, protocols        | `trait Display`                          |
| `variables`       | Module-level variables and constants | `const MAX_SIZE: usize`                  |
| `imports`         | Imports and dependencies             | `use std::fs`                            |

### Markdown Only

| Element       | Description                            | Example                                                                        |
| ------------- | -------------------------------------- | ------------------------------------------------------------------------------ |
| `headings`    | Headings with level, text, and preview | `{ level: 1, title: "Vibe Analyzer", preview: "Universal knowledge base..." }` |
| `links`       | Links                                  | `{ text: "documentation", url: "https://example.com/docs" }`                   |
| `code_blocks` | Code block languages                   | `["bash", "rust"]`                                                             |
| `frontmatter` | YAML metadata                          | `{ title: "...", tags: "...", author: "..." }`                                 |

## Element Support by Language

| Language   | Functions | Classes | Structs | Enums | Interfaces | Variables | Imports |
| ---------- | --------- | ------- | ------- | ----- | ---------- | --------- | ------- |
| Rust       | ✅        | —       | ✅      | ✅    | ✅         | ✅        | ✅      |
| Python     | ✅        | ✅      | —       | —     | —          | ✅        | ✅      |
| JavaScript | ✅        | ✅      | —       | —     | —          | ✅        | ✅      |
| TypeScript | ✅        | ✅      | ✅      | ✅    | ✅         | ✅        | ✅      |
| Java       | ✅        | ✅      | —       | ✅    | ✅         | ✅        | ✅      |
| Go         | ✅        | —       | ✅      | ✅    | —          | ✅        | ✅      |
| C#         | ✅        | ✅      | ✅      | ✅    | —          | ✅        | ✅      |
| Kotlin     | ✅        | ✅      | —       | ✅    | ✅         | ✅        | ✅      |
| Swift      | ✅        | ✅      | ✅      | ✅    | ✅         | ✅        | ✅      |
| Dart       | ✅        | ✅      | —       | ✅    | —          | ✅        | ✅      |
| Bash       | ✅        | —       | —       | —     | —          | ✅        | ✅      |
| Batch      | ✅        | —       | —       | —     | —          | ✅        | ✅      |
| ArkTS      | ✅        | ✅      | ✅      | ✅    | ✅         | ✅        | ✅      |

## Multilingual Search Tags

Each extracted element receives tags in three languages. This allows searching for elements in Russian, English, or Chinese — regardless of the source code language.

| Element Type     | EN                | RU             | ZH       |
| ---------------- | ----------------- | -------------- | -------- |
| Functions        | `functions`       | `функции`      | `函数`   |
| Classes          | `classes`         | `классы`       | `类`     |
| Structs          | `structs`         | `структуры`    | `结构体` |
| Enums            | `enums`           | `перечисления` | `枚举`   |
| Interfaces       | `interfaces`      | `интерфейсы`   | `接口`   |
| Variables        | `variables`       | `переменные`   | `变量`   |
| Imports          | `imports`         | `импорты`      | `导入`   |
| Headings (MD)    | `headings`        | `заголовки`    | `标题`   |
| Code blocks (MD) | `code_blocks`     | `блоки_кода`   | `代码块` |
| Links (MD)       | `links`           | `ссылки`       | `链接`   |
| Module comments  | `header_comments` | —              | —        |

## Doc Comment Formats

Vibe Analyzer extracts documentation from specially formatted comments. Regular comments (`//`, `#`) are ignored.

| Language   | Doc Comment                   | Module Comment                  | Example                                     |
| ---------- | ----------------------------- | ------------------------------- | ------------------------------------------- |
| Rust       | `///` or `/** */`             | `//!` or `/*! */`               | `/// Adds two numbers`                      |
| Python     | `"""..."""` (docstring)       | `"""..."""` at file start       | `"""Adds two numbers"""`                    |
| JavaScript | `/** */` (JSDoc)              | `/** */` at file start          | `/** @param {number} a */`                  |
| TypeScript | `/** */` (JSDoc)              | `/** */` at file start          | `/** @param a First number */`              |
| Java       | `/** */` (Javadoc)            | `/** */` at file start          | `/** @param a First number */`              |
| Kotlin     | `/** */` (KDoc)               | `/** */` at file start          | `/** @param a First number */`              |
| C#         | `///` or `/** */`             | `/** */` or `///` at file start | `/// <summary>Adds two numbers</summary>`   |
| Swift      | `///` or `/** */`             | `///` or `/** */` at file start | `/// - Parameters: a: First number`         |
| Dart       | `///`                         | `///` at file start             | `/// Adds two numbers`                      |
| Go         | `//` (any before declaration) | `//` at file start              | `// Add adds two numbers`                   |
| Bash       | `##` or `#` before function   | `##` at script start            | `## Module documentation for Bash testing`  |
| Batch      | `::` before label             | `::` at script start            | `:: Module documentation for Batch testing` |
| ArkTS      | `/** */`                      | `/** */` at file start          | `/** Async function example */`             |

## AST Example

**Source code (`sample.py`):**

```py
"""
Module documentation for Python testing
"""

import os
import sys
from datetime import datetime
from typing import List, Optional

# Regular comment - ignored


def add(a: int, b: int) -> int:
    """Adds two numbers"""
    return a + b


def multiply(a: int, b: int) -> int:
    """Multiplies two numbers"""
    return a * b


async def fetch_data(url: str) -> str:
    """Async function example"""
    return "data"


class User:
    """User class"""

    def __init__(self, name: str, age: int):
        """Constructor"""
        self.name = name
        self.age = age

    def get_name(self) -> str:
        """Get user name"""
        return self.name


class Config:
    """Config class"""
    debug: bool = False
    max_size: int = 1024


class Color:
    """Color enum (using class constants)"""
    RED = 1
    GREEN = 2
    BLUE = 3


MAX_SIZE: int = 1024
DEFAULT_TIMEOUT: int = 30

APP_NAME: str = "vibe-analyzer"

# Regular comment at the end - ignored
```

**Extracted AST:**

```json
{
  "functions": [
    {
      "signature": "def add(a: int, b: int) -> int",
      "comments": ["Adds two numbers"]
    },
    {
      "signature": "def multiply(a: int, b: int) -> int",
      "comments": ["Multiplies two numbers"]
    },
    {
      "signature": "async def fetch_data(url: str) -> str",
      "comments": ["Async function example"]
    }
  ],
  "classes": [
    {
      "signature": "class User",
      "comments": ["User class"]
    },
    {
      "signature": "class Config",
      "comments": ["Config class"]
    },
    {
      "signature": "class Color",
      "comments": ["Color enum (using class constants)"]
    }
  ],
  "variables": [
    {
      "signature": "MAX_SIZE: int"
    },
    {
      "signature": "DEFAULT_TIMEOUT: int"
    },
    {
      "signature": "APP_NAME: str"
    }
  ],
  "imports": ["os", "sys", "datetime", "typing"],
  "header_comments": ["Module documentation for Python testing"],
  "tags": [
    "header_comments",
    "imports",
    "импорты",
    "导入",
    "variables",
    "переменные",
    "变量",
    "functions",
    "функции",
    "函数",
    "classes",
    "классы",
    "类"
  ]
}
```

## Limitations

- **Maximum file size for AST parsing:** 10 MB (`MAX_AST_FILE_SIZE` constant)
- **Default ignored directories:** `target`, `node_modules`, `__pycache__`, `.venv`, `venv`, `.git`, `.idea`
- **Default ignored files:** `.DS_Store`, `Thumbs.db`, `*.hprof`, `*.log`
- **Binary files:** detected by extension, name, and content analysis, excluded from parsing
- **Nested elements:** methods inside classes are extracted as functions, variables inside functions/classes are not extracted
