#!/usr/bin/env bash
# Purpose: Generate the example index from the published table of contents.
# Usage: Pass the book root or use the matching just recipe.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

src="$(realpath "$1")/src"
index="$src/indices/examples_index.md"
tmp="$(mktemp "$(dirname "$index")/.examples-index.XXXXXX")"
trap 'rm -f "$tmp"' EXIT

# Keep the order and labels readers see in the sidebar.
entries() {
    sed -nE 's/^([[:blank:]]*)-? ?\[(.*)\]\(([^()]+\.md)\)[[:blank:]]*$/\1|\2|\3/p' "$src/SUMMARY.md"
}

{
    # Preserve the human review marker if the existing index has one.
    if [[ "$(head -n 1 "$index")" == '<!-- TODO Reviewed '* ]]; then
        head -n 1 "$index"
        printf '\n'
    fi
    printf '# Index of Examples\n\n'
    section=
    previous_section=
    while IFS='|' read -r indent title chapter; do
        [[ -f "$src/$chapter" && "$chapter" != */_* ]] || continue
        if [[ -z "$indent" ]]; then
            section="$title"
        fi
        table="${chapter%.md}.incl.md"
        has_table=false
        if [[ -f "$src/$table" ]] &&
            rg -q '^\|.*\]\[ex~' "$src/$table" &&
            ! rg -q '^#{1,6}[[:space:]]|\{\{#(rustdoc_)?include[^}]*\.rs([: }])' "$src/$table"; then
            has_table=true
        fi
        scan=("$src/$chapter")
        if [[ -f "$src/$table" ]]; then
            scan+=("$src/$table")
        fi
        if [[ "$has_table" == false ]] &&
            ! rg -q '^[[:space:]]*(\x60{3,}|~{3,})rust\b|\{\{#(rustdoc_)?include[^}]*\.rs([: }])' "${scan[@]}"; then
            continue
        fi
        if [[ "$section" != "$previous_section" ]]; then
            printf '## %s\n\n' "$section"
            previous_section="$section"
        fi
        printf '### [%s](../%s)\n\n' "$title" "$chapter"
        if [[ "$has_table" == true ]]; then
            printf '{{#include ../%s}}\n\n' "$table"
        fi
    done < <(entries)

    printf '{{#include refs.incl.md}}\n{{#include ../refs/link-refs.md}}\n\n'
    sed -n '/^<div class="hidden">/,/^<\/div>/p' "$index"
} > "$tmp"

mv "$tmp" "$index"
