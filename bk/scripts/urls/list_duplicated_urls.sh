#!/usr/bin/env bash
# Purpose: List external URLs that appear repeatedly in the book references.
# Usage: Run from bk/ with just urls list_duplicated.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Identify duplicated URLs (noting that they can't always be avoided).

root="$(realpath $1)/"
echo "URLs that are found more than once in the global references:"
sed -r 's/\[.+?\]:\s*(.+)$/\1/' ${root}src/refs/*.md | sort | uniq --repeated --count
# -r or -E = use extended regular expressions

echo "DONE"
