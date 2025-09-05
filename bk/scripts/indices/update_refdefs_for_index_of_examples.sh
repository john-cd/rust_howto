#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Quick and dirty script
# Add, to `src/refs.incl.md`, missing references that are required for the index of examples (found in `examples_index.md`)
#
# Usage: ./scripts/index_of_examples/update_refdefs_for_index_of_examples.sh <root folder>
#
# This script is idempotent

root="$(realpath $1)/"

# Remove all references to examples
sed -i'.bak' -E '/^\[ex~.+?\]:.*$/d' ${root}src/indices/refs.incl.md

# Look into every refs.incl.md (in the subfolders)
for file in $(find ${root}src -type f -name "refs.incl.md" -not -path "${root}src/indices/refs.incl.md")
do
    echo "Processing ${file}"
    # Get the relative path from `src` to the directory of the current file
    rel=$(realpath --relative-to="${root}src" $file | sed 's/refs.incl.md//');
    # Insert the path in each reference definition
    refs=$(sed -nE 's=(^\[ex~.+?\]:)\s?([^#]+?)(#.+)?$=\1 ../'${rel}'\2\3=p' $file)
    # Add the references to src/indices/refs.incl.md
    echo "${refs}" >> ${root}src/indices/refs.incl.md
done

# Sort and dedupe
sort -u -o ${root}src/indices/refs.incl.md ${root}src/indices/refs.incl.md
# Delete empty lines
sed -i -E '/^\s*$/d' ${root}src/indices/refs.incl.md

echo "DONE"
