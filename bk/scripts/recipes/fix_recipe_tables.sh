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
    # Grab the labels and anchors of the refdefs pointing to the current file from the local references
    # Format is usually [ex~label]: file.md#anchor
    # We will get 'label|anchor' pairs
    labels_and_anchors=$(sed -En "s/^\[ex~([^]]+)\]:\s*(\.\.\/)*${base}(#([^ ]*))?.*$/\1|\4/p" "${dir}/refs.incl.md")

    # If not empty...
    if [ -n "$labels_and_anchors" ]; then
        echo "> ${file}"
        incl_file="${file%.md}.incl.md"

        # Determine number of columns and if crates/categories are present
        num_cols=3
        has_crates=1
        has_categories=1
        if [ -f "$incl_file" ]; then
            header=$(head -n 1 "$incl_file" || true)
            if [[ "$header" == *"| Recipe |"* ]]; then
                num_cols=$(echo "$header" | tr -cd '|' | wc -c || echo 3)
                num_cols=$((num_cols - 1))
                if [[ "$header" != *"Crates"* ]]; then has_crates=0; fi
                if [[ "$header" != *"Categories"* ]]; then has_categories=0; fi
            fi
        fi

        for item in ${labels_and_anchors}
        do
            label="${item%|*}"
            anchor="${item#*|}"

            # If the destination (recipe table) file does not exist or label is not in it
            if [ ! -f "$incl_file" ] || [ $(grep -Pc "\[ex~${label}\]" "$incl_file") -eq 0 ]
            then
                # Extract title, removing only the prefix (like algorithms~) if present, and replace hyphens with spaces
                title=$(echo "${label}" | awk -F'~' '{print $NF}' | tr '-' ' ')

                crates=""
                categories=""

                if [ -n "$anchor" ]; then
                    # Extract the block of text from {#anchor} to the next #
                    block=$(awk "/\{#${anchor}[^}]*\}/ {flag=1} flag && /^#/ && !/\{#${anchor}[^}]*\}/ {flag=0; exit} flag" "$file")
                    crates=$(echo "$block" | grep -o -E '\[\!\[[^]]+\]\[c~[^]]+\]\]\[c~[^]]+\]' | grep -E '~(docs|website|repo)\]' | awk '!seen[$0]++' | tr '\n' ' ' | sed 's/ *$//' || true)
                    categories=$(echo "$block" | grep -o -E '\[\!\[[^]]+\]\[cat~[^]]+\]\]\[cat~[^]]+\]' | awk '!seen[$0]++' | tr '\n' ' ' | sed 's/ *$//' || true)
                fi

                # Build row string based on columns
                row="| [${title}][ex~${label}] |"

                if [ $num_cols -ge 2 ]; then
                    if [ $has_crates -eq 1 ]; then
                        row="$row $crates |"
                    elif [ $has_categories -eq 1 ]; then
                        row="$row $categories |"
                    else
                        row="$row |"
                    fi
                fi

                if [ $num_cols -ge 3 ]; then
                    if [ $has_categories -eq 1 ]; then
                        row="$row $categories |"
                    else
                        row="$row |"
                    fi
                fi

                if [ ! -f "$incl_file" ]; then
                    echo "$row" >> "$incl_file"
                else
                    # Insert before <div class="hidden" if it exists, otherwise append
                    if grep -q "<div class=\"hidden\"" "$incl_file" || grep -q "<div class='hidden'" "$incl_file"; then
                        awk -v row="$row" '/<div class="hidden"/ || /<div class='\''hidden'\''/ {print row; print; next} 1' "$incl_file" > "${incl_file}.tmp"
                        mv "${incl_file}.tmp" "$incl_file"
                    else
                        echo "$row" >> "$incl_file"
                    fi
                fi
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
