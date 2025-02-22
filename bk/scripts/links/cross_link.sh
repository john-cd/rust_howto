#!/usr/bin/env bash
set -ue

# Cross-links between pages
#
# Usage <script>.sh <root folder of book>
# Beware: modifies a lot of files. Manual editing is required after run.
# Does not handle text in ``` ``` blocks correctly.

root="$(realpath $1)/"

files=$(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \))

# Search for potential page-to-page cross-links

# Create an array of filenames (base names)
# Create an associative array base -> orginal filepath
patterns=()
declare -A original_files
for file in ${files}
do
  #echo $'\n'"> ${file}"
  # Look for the file name...
  # - Remove initial _
  # - Use directory name if the file name is index or _index
  base="$( basename -s .md $( sed -E -e 's/_?index.md//' -e 's/other.?\.md//' <<< ${file} ) | sed -E -e 's/^_//' )"
  # Skip very short keyword
  if [ ${#base} -lt 3 ]; then
    continue
  fi
  # Skip common words
  if [ $base == "other" -o $base == "result" -o $base == "time" -o $base == "install" -o $base == "main" -o $base == "building" -o $base == "toml" -o $base == "crates" -o $base == "learning" ]; then
    continue
  fi
  pattern="$(tr '_' ' ' <<< "$base")"
  patterns+=("${pattern}")
  original_files[${pattern}]="$file"
done

# Sort by string length, longest first
IFS=$'\n' sorted=($(printf '%s\n' "${patterns[@]}" | awk '{ print length($0)" "$0 }' | sort -r -n -s | cut -d" " -f2-))
# sort -r = reverse -n = numeric -u = unique -s = stable
unset IFS

for value in "${original_files[@]}"
do
     echo $value
done

for pattern in "${sorted[@]}"
do
  echo ">> Pattern: ${pattern}"
  # Search in all book Markdown files, except a few categories
  # - Ignore patterns in {{#include }} statements, Markdown links / headings, filenames, etc
  # rg -l = Print the paths with at least one match; rg -P = PRCE engine; -t = file type; -g = glob
  start_line='(^[^|[#{]*?[^a-zA-Z#/`["_=@.,:<-])'
  after='([^a-zA-Z"}/._-])'
  #echo "${start_line}${pattern}${after}"
  for file_with_pattern in $(rg -t md -P -l -i -t md -g '!*.incl.md' -g '!*SUMMARY.md' -g '!*index.md' -g'!*refs.md' -g'!*crates*' "${start_line}(${pattern})${after}" "${root}src" )
  do
    # Do not self-reference
    if [ "$file_with_pattern" == "${original_files[$pattern]}" ]; then
      continue
    fi
    echo ">>> File with pattern: $file_with_pattern"
    with_dash=$( tr ' ' '-' <<< "${pattern}" )
    sed -E -i "s~${start_line}(${pattern})${after}~\1[\2][p-${with_dash}]\3~gI" "${file_with_pattern}" # -n  p
    # Add reference to proper refs.incl.md
    dir=$(dirname $file_with_pattern)
    echo ">>>> Original file: ${original_files[$pattern]}"
    echo "[p-${with_dash}]: $(realpath --relative-to=${dir} ${original_files[$pattern]})" >> "${dir}"'/refs.incl.md'
    sort -u -o "${dir}"'/refs.incl.md' "${dir}"'/refs.incl.md'
  done
done

echo "DONE"
