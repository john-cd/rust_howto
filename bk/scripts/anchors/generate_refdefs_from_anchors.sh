#!/usr/bin/env bash
set -euo pipefail

# (BEWARE: modifies files directly) Generate reference definitions from book heading anchors e.g. {#some-text} and add them to the local references in `refs.incl.md`
# Usage: ./scripts/anchors/generate_refdefs_from_anchors.sh <root folder>

root="$(realpath $1)/"

# Anchors should only appear in subchapters, where the examples live.
for file in $(find "${root}src" "${root}drafts" -type f \( -name "*.md" -not -name "*index.md" -not -name "*.incl.md" -not -name "*-refs.md" \) )
do
    echo ">> $file"
    base=$(basename $file)
    dir=$(dirname $file)
    # Get parent directory
    parent=$(dirname $file | xargs basename)
    # Grab all headings with an anchor e.g. {#some_text} - substitute the anchor \1 into a refdef
    link=$(sed -nE 's/^#.*\{#(.+?)\}\s*$/[ex-'${parent}'-\1]: '${base}'#\1/p' ${file})
    if [ -n "$link" ]; then
        echo "$link" >> "${file%/*}/refs.incl.md"
        # remove {#skip}, {#skip1}... and empty lines
        sed -E -i '/(.+?-skip[0-9]*\].*)/d; /^\s*$/d' "${dir}/refs.incl.md"
        # Sort and dedupe refdefs
        sort -u -o "${dir}/refs.incl.md" "${dir}/refs.incl.md"
    fi
done

echo "DONE"
