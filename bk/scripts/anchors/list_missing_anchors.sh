#!/usr/bin/env bash
# Purpose: List book headings that do not have an anchor (e.g. {#some-text} after the heading).
# Usage: ./scripts/anchors/list_missing_anchors.sh <root folder>.
#
# TODO Reviewed ??-??-2026

set -eu

# List book headings that do not have an anchor (e.g. {#some-text} after the heading).
# Note that not all headers need one.

root="$(realpath $1)/"

for file in $(find ${root}src ${root}drafts -type f \( -name "*.md" -not -name "*index.md" -not -name "SUMMARY.md" -not -wholename "${root}*/indices/*.md" \) )
do
    # Grab headings without {, ignoring "## See also", etc...
    header=$(grep -P '^#{2,}[^{]+$' $file | sed -E 's/#{2,}\s+(See [aA]lso|Useful [lL]inks|Reference[s]?|Related Topics)//g')
    if [ -n "$header" ]; then
        echo ">> ${file}
${header}"
    fi
done

echo "DONE"
