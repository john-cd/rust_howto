#! /bin/bash

# BEWARE: modifies files directly
#
# Add, to the recipe tables / local TOCs (in `<subchapter>.incl.md` files), any missing links to recipes,
# using the local reference definitions that have been added to `refs.incl.md` in the same folder.
# Manual editing of the table is still necessary afterwards.
# Use after updating `refs.incl.md`.
#
# Usage: ./scripts/recipe_tables/fix_recipe_tables.sh


for file in $(find ./src -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md")
do
base=$(basename $file)
dir=$(dirname $file)
if [ -f "${dir}/refs.incl.md" ]; then
    # grab the labels of the refdefs for the current file
    labels=$(sed -En 's/^\[ex-(.*)\]:\s?'${base}'.*$/\1/p' ${dir}/refs.incl.md)
    # if not empty
    if [ -n "$labels" ]; then
        echo "> ${file}"
        for label in ${labels}
        do
        # if the dest file does not exist or label is not in it
        if [ ! -f "${file%.md}.incl.md" ] || [ $(grep -Pc "\[ex-${label}\]" "${file%.md}.incl.md") -eq 0 ]
        then
            # Add link in the corresponding .incl.md
            echo "[${label}][ex-${label}]" >> "${file%.md}.incl.md"
        fi
        done
    fi
fi
done
