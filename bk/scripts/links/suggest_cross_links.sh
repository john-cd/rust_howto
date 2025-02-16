#!/usr/bin/env bash
set -ue

# Insert crate links or cross-links between pages
#
# Beware: modifies a lot of files.
# Usage <script>.sh <root folder of book>

root="$(realpath $1)/"

files=$(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \))

# # Insert potential links to crates into the book's Markdown
# for file in ${files}
# do
#   echo ">> ${file}"
#   # Search the Markdown for `word` that is not in a link (i.e. not surrounded by [ ]) and not ```-quoted.
#   # These are potential crate names.
#   # -N = --no-line-number -I = --no-filename -r = replace -i = ignore case
#   for in_backticks in $(rg -INioP '(?<![\[`])`([^ `\[\]]+?)`(?![\]`])' -r '$1' $file | sort -u)
#   do
#     if [[ ${#in_backticks} -gt 4 ]]; then
#       underscored=$(echo ${in_backticks} | tr '-' '_')
#       if [ ${underscored} != ${in_backticks} ]; then
#         pattern="(${in_backticks}|${underscored})"
#       else
#         pattern="${in_backticks}"
#       fi
#       echo "Pattern: ${pattern}"
#       # Find the reference, if any, and create the links, separated by spaces.
#       links=$(rg -INio --null-data '\[(c-)?([^\]]*'${pattern}'[^-]*)\]:' -r '[`'"${in_backticks}"'`][$1$2]⮳{{hi:'"${in_backticks}"'}}' ${root}src/refs | tr '\0' ' ')
#       if [ -n "${links}" ]; then
#         links=$( sed -E -e 's#(\\|~|&)#\\\1#g' <<< "$links" ) # Escape \ ~ & and newlines
#         echo "Links: ${links}"
#         # Replace `<word>` by the links, but only if the line does not start with # (heading)
#         sed -E -i 's~^([^#].*[^\[`])?`'"${in_backticks}"'`~\1'"${links}"'~g' "${file}" # -n   p
#       else
#         echo "[${in_backticks}]: " | sed -E -e 's/<T>//g' >> suggest.md
#       fi
#     fi
#   done
# done

# sort -u -o suggest.md suggest.md

#echo "SECOND PART"

# Search for potential page-to-page cross-links
for file in ${files}
do
  echo $'\n'"> ${file}"
  # Look for the file name (incl. with _ replaced by -)...
  # Remove initial _
  # Use directory name if index or _index
  base="$( basename -s .md $( sed -E -e 's/_?index.md//' -e 's/other.?\.md//' <<< ${file} ) | sed -E -e 's/^_//' )"
  if [ -z ${base} ]; then
    continue
  fi
  if [ ${#base} -lt 3 ]; then
    continue
  fi
  if [ $base == "other" -o $base == "result" -o $base == "time" -o $base == "install" -o $base == "main" -o $base == "building" -o $base == "toml" -o $base == "crates" -o $base == "learning" ]; then
    continue
  fi
  with_space=$(tr '_-' ' ' <<< $base)
  if [ "${with_space}" != "${base}" ]; then
        pattern="(${base}|${with_space})"
      else
        pattern="(${base})"
  fi
  echo ">> Pattern: $pattern"
  # ...in all book Markdown files, except a few categories
  # Ignore patterns in {{#include }} statements
  # -l = Print the paths with at least one match; -P = PRCE engine; -t = file type; -g = glob
  start_line='(^[^|[#{]*?[^a-zA-Z#/`["_=@.,:<-])'
  after='([^a-zA-Z"}/._-])'
  #echo "${start_line}${pattern}${after}"
  for file_with_pattern in $(rg -t md -P -l -i -t md -g '!*.incl.md' -g '!*SUMMARY.md' -g '!*index.md' -g'!*refs.md' -g'!*crates*' "${start_line}${pattern}${after}" "${root}src" )
  do
    # Do not self-reference
    if [ "$file_with_pattern" = "$file" ]; then
      #echo ">>>> Same file - skip"
      continue
    fi
    echo ">>> File with pattern: $file_with_pattern"
    with_dash=$( tr '_' '-' <<< ${base} )
    sed -E -i "s~${start_line}${pattern}${after}~\1[\2][p-${with_dash}]\3~gI" "${file_with_pattern}" # -n  p
    # Add reference to proper refs.incl.md
    dir=$(dirname $file_with_pattern)
    echo "[p-${with_dash}]: $(realpath --relative-to=${dir} ${file})" >> "${dir}"'/refs.incl.md'
    sort -u -o "${dir}"'/refs.incl.md' "${dir}"'/refs.incl.md'
  done
done

echo "DONE"



# excluding suffixes like -github, -badge, etc... (which start with a dash)
# To only avoid matching lines that contain -badge use (?:(?!-badge)[^]])* with the --pcre2 flag


# # fe [FUZZY PATTERN]
# # - Bypass fuzzy finder if there's only one match (--select-1)
# # - Exit if there's no match (--exit-0)
# fe() {
#   IFS=$'\n' files=($(fzf --query="$1" --multi --select-1 --exit-0 --preview="{}"))
# }
