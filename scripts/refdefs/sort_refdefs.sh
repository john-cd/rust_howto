#! /bin/bash

# Usage: ./scripts/refdefs/sort_refdefs.sh

# Sort and deduplicate reference definitions in the central `*-refs.md` files

sort -u -o ./src/refs/crate-refs.md ./src/refs/crate-refs.md
sort -u -o ./src/refs/other-refs.md ./src/refs/other-refs.md
sort -u -o ./src/refs/link-refs.md ./src/refs/link-refs.md
# category-refs.md should not be sorted

# Remove the last / from URLs in the reference definition files

sed -i 's/[/]$//g' ./src/refs/crate-refs.md
sed -i 's/[/]$//g' ./src/refs/other-refs.md
sed -i 's/[/]$//g' ./src/refs/link-refs.md
sed -i 's/[/]$//g' ./src/refs/category-refs.md

echo "DONE"
