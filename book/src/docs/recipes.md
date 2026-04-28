# Recipes

Practical examples of using Vibe Analyzer through an AI assistant.

## Exploring an Unfamiliar Project

**Goal:** you're seeing a project for the first time and want to quickly understand its structure.

**Assistant dialog:**

```
You: What projects are in the database?
AI: [show_projects] Project "backend" available — payment processing service.

You: Show the structure of backend
AI: [show_tree] backend/
├── src/
│   ├── main.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── payments.rs
│   │   └── refunds.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── models.rs
│   └── config.rs
├── migrations/
├── tests/
└── Cargo.toml

You: Show statistics for backend
AI: [show_stats]
Rust        45 files    8,200 lines    420 AST objects
SQL         12 files    1,500 lines      —
Markdown     5 files      300 lines     25 AST objects
Total       62 files   10,000 lines    445 AST objects

You: What are the main functions in api/payments.rs?
AI: [get_file_ast for api/payments.rs]
- process_payment(amount, currency)
- validate_card(card_number)
- create_transaction(order_id)
- refund_payment(transaction_id)
```

## Finding Dependencies

**Goal:** find out what libraries are used in a project.

```
You: What libraries are used in backend?
AI: [search_by_code_imports]
- serde (serialization)
- tokio (async runtime)
- sqlx (PostgreSQL)
- reqwest (HTTP client)
- stripe (payment API)

You: Where is stripe used?
AI: [search_by_code_imports with query="stripe"]
- src/api/payments.rs
- src/api/refunds.rs
- src/api/webhooks.rs
```

## Finding an Implementation

**Goal:** find where a specific function is implemented.

```
You: Find the calculate_total function
AI: [search_by_code_functions with query="calculate_total"]
Found in src/orders.rs:
  pub fn calculate_total(items: &[OrderItem]) -> f64
  "Calculates total order amount with discounts applied"
```

## Documentation Search

**Goal:** find rules or processes.

```
You: Find Python coding guidelines
AI: [search_documentation with query="Python coding guidelines"]
Found in docs/guidelines/python.md:
  # Python Coding Guidelines
  Follow PEP 8. Use snake_case, type annotations, docstrings.

You: What is the release process?
AI: [search_documentation with query="release process"]
Found in docs/process/release.md:
  # Release Process
  1. All milestone tasks closed
  2. Create a release branch from develop
  3. Update version in Cargo.toml
  4. Merge into main
  5. Notify the team
```

## Knowledge Base Search

**Goal:** find information about project characters or legends.

```
You: Who is Kirikur?
AI: [search_documentation with query="Kirikur"]
Found in knowledge/legends/kirikur_programmer.md:
  # Kirikur - Programmer
  Backend developer, stack: Rust, Python, PostgreSQL.
  Works at a food delivery startup.
  Rewrote legacy code in Rust in 3 months.
  Favorite phrase: "It works on my machine"

You: What is Bakobur's dream?
AI: [search_documentation with query="Bakobur's dream"]
Found in knowledge/legends/bakobur_homeless.md:
  Dream: Warm socks and a doghouse for Sharik
```

## Incremental Update

**Goal:** update the index after code changes.

```
You: Update the index
AI: [admin_sync]
Indexing started. Projects are updating.

# After some time, you can check:
You: Show backend statistics
AI: [show_stats]
Rust        47 files    8,500 lines    445 AST objects
...
```

## Inspecting a Specific File

**Goal:** view the contents or structure of a specific file.

```
You: Show the contents of Cargo.toml
AI: [get_file_content]
[package]
name = "backend"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }

You: What structs are in db/models.rs?
AI: [get_file_ast]
- struct Order { id, user_id, status, total, created_at }
- struct OrderItem { id, order_id, product_id, quantity, price }
- enum OrderStatus { Pending, Confirmed, Shipped, Delivered, Cancelled }
```

## Tips

- **Start broad** — `show_projects` → `show_tree` → `show_stats`
- **Refine with search** — `search_by_code_functions`, `search_documentation`
- **Inspect details** — `get_file_content`, `get_file_ast`
- **Update the index after changes** — `admin_sync`
- **Use natural language** — the model will choose the right tool automatically
