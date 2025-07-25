#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Make the titles in the recipe tables / local TOCs match the corresponding headings of the subchapters
#
# Usage: ./scripts/recipe_tables/check_recipe_tables.sh <root folder>

root="$(realpath $1)/"
for file in $(find ${root}src ${root}drafts -type f -name "*.md" -not -name "*.incl.md" -not -name "*index.md" -not -name "*refs.md" -not -name "TOREVIEW.md")
do
    echo ">>${file}"
    name=$(basename $file .md)
    dir=$(dirname $file)
    # Get title and anchor of all headings in the subchapter
    titles_and_anchors=$(sed -En 's=^#+\s*([^\{]+)\{#([^\}]+)\}\s*$=\1@\2=p' $file)
    for taa in ${titles_and_anchors}
    do
      if [ -f "${dir}/${name}.incl.md" ]; then
        title=$(echo "$taa" | cut -d"@" -f1 | sed -E -e 's/[[:space:]]*$//' -e 's/^[[:space:]]*//' -e 's=&=\&=g')
        anchor=$(echo "$taa" | cut -d"@" -f2 | sed -E -e 's=&=\&=g')
        #echo "-->${title}< >${anchor}<"
        sed -i -E 's=^\|\s*\[[^]]+\](\[ex~[^]]+'"${anchor}"'\].*)$=| ['"${title}"']\1=' "${dir}/${name}.incl.md"
        # -i = subsitute in place -E = extended regex
        # matches [...][ex~...{anchor}]... and replaces the current label by the subchapter title
      fi
    done
done

echo "DONE"

# [escape & in titles; combine with the generate / fix recipe table script in titles; combine with the generate / fix recipe table script](https://github.com/john-cd/rust_howto/issues/1373)
