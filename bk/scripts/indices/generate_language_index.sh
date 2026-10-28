#!/usr/bin/env bash
# Purpose: Generate a template for the book language index.
# Usage: Run from bk/ with just indices generate_language_index.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Quick and dirty script
# Outputs a template for `language/index.md`
# Manual editing required afterwards
#

# Print header
echo '| Language Constructs |
|--------|'

root="$(realpath $1)/"
# Read the markdown files in the `language` section
for file in $(find ${root}src/language -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "index.md")
do
    base=$(basename $file)
    # Title case the filename, replace _ by space and keep conjunctions lowercase
    title=$(echo ${base%.md} | sed 's/.*/\L&/; s/[a-z]*/\u&/g; s/_/ /g; s/And/and/g')
    echo "| [$title][ex~language~${base%.md}] |"
done
