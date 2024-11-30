#! /bin/bash

# Make sure that a local TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
#
# Usage: ./scripts/includes/list_missing_subchapter_includes.sh

for file in $(find ./src -type f -name "*.md" -not -name "*index.md" -not -name '*.incl.md' -not -name "*refs.md"  -not -path "./src/[^/]+.md" )
do
  base=$(basename $file)
  include=$(grep -Poh '(?<=\{\{#include )_?'${base%.md}'(?=\.incl\.md\}\})' $file)
  if [ -z "$include" ]; then
    echo $file" -- consider adding --> {{#include "${base%.md}".incl.md}}"
  fi
done
