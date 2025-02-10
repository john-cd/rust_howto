#! /bin/bash
set -u

# List subchapters that do not include a local TOC, i.e. {{#include <subchapter>.incl.md}} is missing.
#
# Usage: ./scripts/includes/list_missing_subchapter_includes.sh <root folder>

root="$(realpath $1)/"
for file in $(find ${root}src -type f -name "*.md" -not -name "*index.md" -not -name '*.incl.md' \
              -not -name "*refs.md" -not -path "${root}src/*.md" -not -path "${root}src/crates/*.md" )
do
  base=$(basename $file)
  include=$(grep -Poh '(?<=\{\{#include )_?'${base%.md}'(?=\.incl\.md\}\})' $file)
  if [ -z "$include" ]; then
    echo $file" -- consider adding --> {{#include "${base%.md}".incl.md}}"
  fi
done

echo "DONE"
