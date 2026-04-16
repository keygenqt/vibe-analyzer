#!/bin/bash

set -e

echo "=> Test vibe-analyzer..."
RUSTFLAGS="-C link-arg=-Wl,--allow-multiple-definition" cargo test
# cargo build --profile release-linux --verbose
# cp target/release-linux/vibe-analyzer /usr/local/bin/

# echo "=> Waiting for OpenSearch..."
# until curl -s http://opensearch:9200 > /dev/null; do
#     sleep 2
# done
# echo "=> OpenSearch is ready"

# echo "=> Indexing fixtures..."
# vibe-analyzer scan index
