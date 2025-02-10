#! /bin/bash
set -euo pipefail
IFS=$'\n\t'

# Make the titles in the recipe tables / local TOCs match the corresponding headings of the subchapters
#
# Usage: ./scripts/recipe_tables/check_recipe_tables.sh <root folder>

root="$(realpath $1)/"
for file in $(find ${root}src -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md")
do
    echo ">>${file}"
    name=$(basename $file .md)
    dir=$(dirname $file)
    # Get title and anchor of all headings in the subchapter
    titles_and_anchors=$(sed -En 's~^#+\s*([^\{]+)\{#([^\}]+)\}\s*$~\1@\2~p' $file)
    for taa in ${titles_and_anchors}
    do
        title=$(echo "$taa" | cut -d"@" -f1 | sed -e 's/[[:space:]]*$//' -e 's/^[[:space:]]*//')
        anchor=$(echo "$taa" | cut -d"@" -f2)
        #echo "-->${title}< >${anchor}<"
        sed -i -E 's~^\|\s*\[[^]]+\](\[ex-[^]]+'${anchor}'\].*)$~| ['${title}']\1~' "${dir}/${name}.incl.md"
        # -i = subsitute in place -E = extended regex
        # matches [...][ex-...{anchor}]... and replaces the current label by the subchapter title
    done
done

echo "DONE"
