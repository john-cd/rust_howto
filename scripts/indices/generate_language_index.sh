#! /bin/bash
set -euo pipefail

# Quick and dirty script
# Outputs a template for `language/index.md`
# Manual editing required afterwards
#
# Usage: /code/scripts/language/generate_language_index.sh >> /code/src/language/index.incl.md

# Print header
echo '| Language Constructs |
|--------|'

root="/code/"
# Read the markdown files in the `language` section
for file in $(find ${root}src/language -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "index.md")
do
    base=$(basename $file)
    # Title case the filename, replace _ by space and keep conjunctions lowercase
    title=$(echo ${base%.md} | sed 's/.*/\L&/; s/[a-z]*/\u&/g; s/_/ /g; s/And/and/g')
    echo "| [$title][ex-language-${base%.md}] |"
done
