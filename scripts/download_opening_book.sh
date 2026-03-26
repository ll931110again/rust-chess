#!/usr/bin/env bash
set -euo pipefail

BOOK_URL="${1:-https://raw.githubusercontent.com/joeyrobert/ceruleanjs_opening_books/master/komodo.bin}"
OUT_PATH="${2:-books/komodo.bin}"

mkdir -p "$(dirname "$OUT_PATH")"
curl -L --fail --output "$OUT_PATH" "$BOOK_URL"
echo "Downloaded opening book to $OUT_PATH"
