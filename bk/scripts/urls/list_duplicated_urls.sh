#!/usr/bin/env bash
set -euo pipefail

# Identify duplicated URLs (noting that they can't always be avoided).
# Usage: ./scripts/urls/list_duplicated_urls.sh

root="$(realpath $1)/"
echo "URLs that are found more than once in the global references:"
sed -r 's/\[.+?\]:\s*(.+)$/\1/' ${root}src/refs/*.md | sort | uniq --repeated --count
# -r or -E = use extended regular expressions

echo "DONE"
