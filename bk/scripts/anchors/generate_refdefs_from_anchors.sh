#!/usr/bin/env bash
set -euo pipefail

# (BEWARE: modifies files directly) Generate reference definitions from book heading anchors e.g. {#some-text} and add them to the local references in `refs.incl.md`
# Usage: ./scripts/anchors/generate_refdefs_from_anchors.sh <root folder>

root="$(realpath $1)/"

# Remove previous refdefs.
for dir in $(find "${root}src" "${root}drafts" "${root}later" -type d)
do
    ref_file="${dir}/refs.incl.md"
    if [ -f "${ref_file}" ]; then
      sed -E -i '/^\[ex~.*$/d; /^\s*$/d' "${ref_file}" || true
    fi
done

# Anchors should only appear in subchapters, where the examples live.
for file in $(find "${root}src" "${root}drafts" "${root}later" -type f \( -name "*.md" -not -name "*index.md" -not -name "*.incl.md" -not -name "*-refs.md" \) )
do
    echo ">> $file"
    base=$(basename $file)
    dir=$(dirname $file)
    ref_file="${dir}/refs.incl.md"
    # Get the parent directory's name.
    parent=$(dirname $file | xargs basename)

    # Grab all headings with an anchor e.g. {#some_text} - substitute the anchor \1 into a refdef.
    link=$(sed -nE 's/^#.*\{#(.+?)\}\s*$/[ex~'${parent}'~\1]: '${base}'#\1/p' ${file})
    if [ -n "$link" ]; then
        echo "$link" >> "${ref_file}"
        # Remove {#skip}, {#skip1}... and empty lines.
        sed -E -i '/(.+?~skip[0-9]*\].*)/d; /(.+?~related-topics\].*)/d; /(.+?~references\].*)/d; /^\s*$/d' "${ref_file}"
        # Sort and dedupe refdefs.
        sort -u -o "${dir}/refs.incl.md" "${ref_file}"
    fi
done

echo "DONE"
