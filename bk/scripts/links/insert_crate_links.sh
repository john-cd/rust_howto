#!/usr/bin/env bash
set -u

# Insert crate links
#
# Usage <script>.sh <root folder of book>
# Beware: modifies a lot of files. Manual editing is required after run.
# Does not handle text in ``` ``` blocks correctly.

root="$(realpath $1)/"

# Markdown files
files=$(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \))

# Insert potential links to crates into the book's Markdown
for file in ${files}
do
  echo ">> ${file}"
  # Search the Markdown for `word` that is not in a link (i.e. not surrounded by [ ]) and not ```-quoted.
  # These are potential crate names.
  # -N = --no-line-number -I = --no-filename -r = replace -i = ignore case
  for in_backticks in $(rg -INioP '(?<![\[`])`([^ `\[\]]+?)`(?![\]`])' -r '$1' $file | sort -u)
  do
    if [[ ${#in_backticks} -gt 1 ]]; then
      trimmed="${in_backticks/%[\._-]rs/}" # remove .rs or -rs or _rs
      if [ ${trimmed} != ${in_backticks} ]; then
        pattern="${in_backticks}|${trimmed}"
      else
        pattern="${in_backticks}"
      fi
      underscored=$(echo ${pattern} | tr '-' '_')
      if [ ${underscored} != ${pattern} ]; then
        pattern="(${pattern}|${underscored})"
      else
        pattern="(${pattern})"
      fi
      echo "Pattern: ${pattern}"
      # Find the reference, if any, and create the link(s), separated by spaces if multiple. Trim last space.
      # -I = no filename; -N = no line number; -o = only-matching; -i = ignore case
      links=$(rg -INio --null-data '\[(c-'"${pattern}"')\]:' -r '[`'"${in_backticks}"'`][$1]⮳{{hi:'"${in_backticks}"'}}' ${root}src/refs/crate-refs.md | tr '\0' ' ' | sed -E -e 's/ +$//' )
      if [ -n "${links}" ]; then
        links=$( sed -E -e 's#(\\|~|&)#\\\1#g' <<< "$links" ) # Escape \ ~ &
        echo "Links: ${links}"
        # Replace `<word>` by the links, but only if the line does not start with # (heading) and not in a link / ```...``` region
        sed -E -i 's~^([^#].*[^\[`])?`'"${in_backticks}"'`~\1'"${links}"'~g' "${file}" # -n   p
      else
        # Add potentially missing crates to a file.
        echo "${in_backticks}" | sed -E -e 's/<T>//g' >> ${root}suggest.md
      fi
    fi
  done
done

# Sort and make unique
sort -u -o ${root}suggest.md ${root}suggest.md

echo "DONE"
