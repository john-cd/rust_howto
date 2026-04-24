#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# BEWARE: modifies files directly
#
# Add, to the recipe tables / local TOCs (in `<subchapter>.incl.md` files), any missing links to recipes,
# using the local reference definitions that have been added to `refs.incl.md` in the same folder.
# Manual editing of the table is still necessary afterwards.
# Use after updating `refs.incl.md`.
# Also makes the titles in the recipe tables / local TOCs match the corresponding headings of the subchapters.
#
# Usage: ./scripts/recipe_tables/fix_recipe_tables.sh <root folder>

root="$(realpath $1)/"
# Iterate through subchapters
for file in $(find ${root}src ${root}drafts -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md" -not -name "TOREVIEW.md")
do
base=$(basename $file)
name=$(basename $file .md)
dir=$(dirname $file)
if [ -f "${dir}/refs.incl.md" ]; then
    # Grab the labels of the refdefs pointing to the current file from the local references
    labels=$(sed -En 's/^\[ex~(.*)\]:\s?'${base}'.*$/\1/p' ${dir}/refs.incl.md)
    # If not empty...
    if [ -n "$labels" ]; then
        echo "> ${file}"
        for label in ${labels}
        do
        # If the destination (recipe table) file does not exist or label is not in it
        if [ ! -f "${file%.md}.incl.md" ] || [ $(grep -Pc "\[ex~${label}\]" "${file%.md}.incl.md") -eq 0 ]
        then
            title=$(echo ${label} | tr '-' ' ')
            # Add table row with link in the corresponding .incl.md
            echo "| [${title}][ex~${label}] | | |" >> "${file%.md}.incl.md"
        fi
        done
    fi
fi

# Get title and anchor of all headings in the subchapter
titles_and_anchors=$(sed -En 's=^#+\s*([^\{]+)\{#([^\}]+)\}\s*$=\1@\2=p' $file)
for taa in ${titles_and_anchors}
do
  if [ -f "${dir}/${name}.incl.md" ]; then
    title=$(echo "$taa" | cut -d"@" -f1 | sed -E -e 's/[[:space:]]*$//' -e 's/^[[:space:]]*//' -e 's/&/\\&/g')
    anchor=$(echo "$taa" | cut -d"@" -f2 | sed -E -e 's/&/\\&/g')
    #echo "-->${title}< >${anchor}<"
    sed -i -E "s=^\|[[:space:]]*\[[^]]*\](\[ex~[^]]*${anchor}\].*)$=| [${title}]\1=" "${dir}/${name}.incl.md"
    # -i = subsitute in place -E = extended regex
    # matches [...][ex~...{anchor}]... and replaces the current label by the subchapter title
  fi
done

done

echo "DONE"

# [append before <div class="hidden" >; insert crate and categories badges; handle tables with only one or two columns](https://github.com/john-cd/rust_howto/issues/1372)
