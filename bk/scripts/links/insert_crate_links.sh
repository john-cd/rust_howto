#!/usr/bin/env bash
# TODO Reviewed ??-??-2026
set -uo pipefail

# Insert crate links
#
# Usage <script>.sh <root folder of book>
# Beware: modifies a lot of files. Manual editing is required after run.
# Does not handle text in ``` ``` blocks correctly.

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <root folder of book>" >&2
  exit 2
fi

root="$(realpath -- "$1")/"
if [[ ! -d "$root/src" || ! -d "$root/drafts" ]]; then
  echo "Error: '$root' must contain src/ and drafts/ directories." >&2
  exit 1
fi

# Markdown files
suggest_file="$root/suggest.md"
touch "$suggest_file"

# Insert potential links to crates into the book's Markdown
while IFS= read -r -d '' file; do
  echo ">> ${file}"
  # Search the Markdown for `word` that is not in a link (i.e. not surrounded by [ ]) and not ```-quoted.
  # These are potential crate names.
  # -N = --no-line-number -I = --no-filename -r = replace -i = ignore case
  while IFS= read -r in_backticks; do
    if [[ ${#in_backticks} -gt 1 ]]; then
      trimmed="${in_backticks/%[\._-]rs/}" # remove .rs or -rs or _rs
      if [ "$trimmed" != "$in_backticks" ]; then
        pattern="${in_backticks}|${trimmed}"
      else
        pattern="${in_backticks}"
      fi
      underscored=$(printf '%s' "$pattern" | tr '-' '_')
      if [ "$underscored" != "$pattern" ]; then
        pattern="(${pattern}|${underscored})"
      else
        pattern="(${pattern})"
      fi
      echo "Pattern: ${pattern}"
      # Find the reference, if any, and create the link(s), separated by spaces if multiple. Trim last space.
      # -I = no filename; -N = no line number; -o = only-matching; -i = ignore case
      links=$(rg -INio --null-data '\[(c-'"${pattern}"')\]:' -r '[`'"${in_backticks}"'`][$1]↗{{hi:'"${in_backticks}"'}}' "$root/src/refs/crate-refs.md" | tr '\0' ' ' | sed -E -e 's/ +$//' )
      if [ -n "${links}" ]; then
        links=$( sed -E -e 's#(\\|~|&)#\\\1#g' <<< "$links" ) # Escape \ ~ &
        echo "Links: ${links}"
        # Replace `<word>` by the links, but only if the line does not start with # (heading) and not in a link / ```...``` region
        sed -E -i 's=^([^#].*[^\[`])?`'"${in_backticks}"'`=\1'"${links}"'=g' "${file}" # -n p
      else
        # Add potentially missing crates to a file.
        echo "$in_backticks" | sed -E -e 's/<T>//g' >> "$suggest_file"
      fi
    fi
  done < <(rg -INioP '(?<![\[`])`([^ `\[\]]+?)`(?![\]`])' -r '$1' -- "$file" | sort -u || true)
done < <(find "$root/src" "$root/drafts" -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \) -print0)

# Sort and make unique
sort -u -o "$suggest_file" "$suggest_file"

echo "DONE"
