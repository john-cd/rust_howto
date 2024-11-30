#! /bin/bash

# List book headings that do not have an anchor (e.g. {#some-text} after the heading).
# Note that not all headers need one.
# Usage: ./scripts/anchors/list_missing_anchors.sh

for file in $(find ./src -type f \( -name "*.md" -not -name "*index.md" -not -wholename "./src/crates/*.md" \) )
do
    # Grab headings without {, ignoring "## See also", etc...
    header=$(grep -P '^#{2,}[^{]+$' $file | sed -E 's/#{2,}\s+(See [aA]lso|Useful [lL]inks|Reference[s]?)//g')
    if [ -n "$header" ]; then
        echo ">> ${file}
${header}"
    fi
done

echo "DONE"
