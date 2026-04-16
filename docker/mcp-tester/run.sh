#!/bin/bash
set -e

cd "$(dirname -- "${BASH_SOURCE[0]}")/../../"

# Load config
cd docker/mcp-tester
if [ -f .env ]; then
    source .env
else
    echo "=> Creating .env from example..."
    cp env.example .env
    source .env
fi
cd ../..

# Verify Ollama
echo "=> Checking Ollama on port $OLLAMA_HOST_PORT..."
curl -s "http://localhost:$OLLAMA_HOST_PORT/api/tags" > /dev/null 2>&1 || {
    echo "   ERROR: Ollama not running"
    exit 1
}

# Pull models
echo "=> Pulling models..."
for model in "qwen2.5-coder:3b-instruct" "qwen2.5-coder:14b-instruct"; do
    ollama pull "$model" 2>/dev/null && echo "   OK: $model" || true
done

# Run tests
echo "=> Starting tests..."
cd docker/mcp-tester
docker compose down --volumes 2>/dev/null || true
docker compose up

# echo "=> Test results:"
# docker compose logs test-runner | tail -50

# docker compose down
