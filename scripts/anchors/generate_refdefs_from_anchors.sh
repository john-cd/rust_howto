#! /bin/bash

# (BEWARE: modifies files directly) Generate reference definitions from book heading anchors e.g. {#some-text} and add them to the local references in `refs.incl.md`
# Usage: ./scripts/anchors/generate_refdefs_from_anchors.sh

for file in $(find ./src -type f \( -name "*.md" -not -name "*index.md" \) )
do
    echo ">> $file"
    base=$(basename $file)
    # Get parent directory
    parent=$(dirname $file | xargs basename)
    # Grab all headings with an anchor e.g. {#some_text} - substitute the anchor \1 into a refdef
    link=$(sed -nE 's/^#.*\{#(.+?)\}\s*$/[ex-'${parent}'-\1]: '${base}'#\1/p' ${file})
    if [ -n "$link" ]; then
        echo "$link" >> "${file%/*}/refs.incl.md"
        # Sort and dedupe refdefs
        sort -u -o "${file%/*}/refs.incl.md" "${file%/*}/refs.incl.md"
    fi
done

echo "DONE"
