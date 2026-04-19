#!/usr/bin/env bash
# Enforce the 1000-line file-length rule for handwritten Rust source.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

violations=""
while IFS= read -r file; do
    [ -z "$file" ] && continue
    lines=$(wc -l < "$file")
    if [ "$lines" -gt 1000 ]; then
        violations="$violations$file:$lines\n"
    fi
done < <(find crates apps -path '*/target' -prune -o -name '*.rs' -type f -print)

if [ -n "$violations" ]; then
    echo "Rust source files over 1000 lines:"
    printf "%b" "$violations"
    exit 1
fi

echo "All Rust source files are within the 1000-line budget."
