#!/usr/bin/env bash
# Search the locally-generated Bevy 0.19 docs.
# Usage: ./tools/search-docs.sh <query>

set -euo pipefail

QUERY="${1:-}"
if [[ -z "$QUERY" ]]; then
    echo "Usage: $0 <search-term>"
    exit 1
fi

DOC_ROOT="$(cd "$(dirname "$0")/.." && pwd)/target/doc"

if [[ ! -d "$DOC_ROOT/bevy" ]]; then
    echo "Docs not found at $DOC_ROOT/bevy"
    echo "Run: cargo doc"
    exit 1
fi

echo "Searching Bevy docs for: $QUERY"
rg -i "$QUERY" "$DOC_ROOT/bevy" "$DOC_ROOT"/bevy_*
