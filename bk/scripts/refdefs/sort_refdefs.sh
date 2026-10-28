#!/usr/bin/env bash
# Purpose: Sort and deduplicate reference definitions in the central `*-refs.md` files.
# Usage: ./scripts/refdefs/sort_refdefs.sh <root folder>.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail


# Sort and deduplicate reference definitions in the central `*-refs.md` files

root="$(realpath -- "$1")/"
sort -u -o "${root}src/refs/crate-refs.md" "${root}src/refs/crate-refs.md"
sort -u -o "${root}src/refs/other-refs.md" "${root}src/refs/other-refs.md"
sort -u -o "${root}src/refs/link-refs.md" "${root}src/refs/link-refs.md"
# category-refs.md should not be sorted

# Remove the last / from URLs in the reference definition files

sed -i 's/[/]$//g' "${root}src/refs/crate-refs.md"
sed -i 's/[/]$//g' "${root}src/refs/other-refs.md"
sed -i 's/[/]$//g' "${root}src/refs/link-refs.md"
sed -i 's/[/]$//g' "${root}src/refs/category-refs.md"

echo "DONE"
