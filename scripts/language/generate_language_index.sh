#! /bin/bash

# Quick and dirty generation of `language/index.md``; manual editing required afterwards
# Usage: ./scripts/language/generate_language_index.sh >> src/language/index.incl.md

clear
echo '| Language Constructs |
|--------|'
for file in $(find ./src/language -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "index.md")
do
    base=$(basename $file)
    title=$(echo ${base%.md} | sed 's/.*/\L&/; s/[a-z]*/\u&/g; s/_/ /g; s/And/and/g;')
    echo "| [$title][ex-lang-${base%.md}] |"
done
