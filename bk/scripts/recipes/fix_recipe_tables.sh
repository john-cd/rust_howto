#!/usr/bin/env bash
set -euo pipefail

# BEWARE: modifies files directly
#
# Add, to the recipe tables / local TOCs (in `<subchapter>.incl.md` files), any missing links to recipes,
# using the local reference definitions that have been added to `refs.incl.md` in the same folder.
# Manual editing of the table is still necessary afterwards.
# Use after updating `refs.incl.md`.
#
# Usage: ./scripts/recipe_tables/fix_recipe_tables.sh <root folder>

root="$(realpath $1)/"
# Iterate through subchapters
for file in $(find ${root}src ${root}drafts -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md")
do
base=$(basename $file)
dir=$(dirname $file)
if [ -f "${dir}/refs.incl.md" ]; then
    # Grab the labels of the refdefs pointing to the current file from the local references
    labels=$(sed -En 's/^\[ex-(.*)\]:\s?'${base}'.*$/\1/p' ${dir}/refs.incl.md)
    # If not empty...
    if [ -n "$labels" ]; then
        echo "> ${file}"
        for label in ${labels}
        do
        # If the destination (recipe table) file does not exist or label is not in it
        if [ ! -f "${file%.md}.incl.md" ] || [ $(grep -Pc "\[ex-${label}\]" "${file%.md}.incl.md") -eq 0 ]
        then
            title=$(echo ${label} | tr '-' ' ')
            # Add table row with link in the corresponding .incl.md
            echo "| [${title}][ex-${label}] | {{#crate }} | {{#categories }} |" >> "${file%.md}.incl.md"
        fi
        done
    fi
fi
done

echo "DONE"

# TODO append before <div class="hidden" >; insert crate and categories badges; handle tables with only one or two columns
