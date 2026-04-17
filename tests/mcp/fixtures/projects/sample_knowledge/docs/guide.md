# Test Guide

This is a sample knowledge base document for testing.

## Installation

To install the project, run:

```bash
cargo build --release
```

## Configuration

Create a config file at `~/.config/app/config.toml`:

```toml
[server]
host = "localhost"
port = 8080
```

## API Reference

### GET /health

Returns service health status.

### POST /query

Send a search query and get results.

## Links

See [official documentation](https://example.com) for more details.

````

Достаточно для проверки:
- Заголовков (`##`)
- Кодовых блоков (```bash, ```toml)
- Ссылок `[text](url)`
- Обычного текста

Создать файл:

```bash
mkdir -p fixtures/sample_knowledge
cat > fixtures/sample_knowledge/guide.md << 'EOF'
# Test Guide

This is a sample knowledge base document for testing.

## Installation

To install the project, run:

```bash
cargo build --release
````

## Configuration

Create a config file at `~/.config/app/config.toml`:

```toml
[server]
host = "localhost"
port = 8080
```

## API Reference

### GET /health

Returns service health status.

### POST /query

Send a search query and get results.

## Links

See [official documentation](https://example.com) for more details.
EOF
