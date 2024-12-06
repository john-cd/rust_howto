#! /bin/bash

# Quick and dirty script
# Outputs a template for `language/index.md`
# Manual editing required afterwards
#
# Usage: ./scripts/language/generate_language_index.sh >> src/language/index.incl.md

# Print header
echo '| Language Constructs |
|--------|'

# Read the markdown files in the `language` section
for file in $(find ./src/language -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "index.md")
do
    base=$(basename $file)
    # Title case the filename, replace _ by space and keep conjunctions lowercase
    title=$(echo ${base%.md} | sed 's/.*/\L&/; s/[a-z]*/\u&/g; s/_/ /g; s/And/and/g')
    echo "| [$title][ex-language-${base%.md}] |"
done
