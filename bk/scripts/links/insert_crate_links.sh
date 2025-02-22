#!/usr/bin/env bash
set -ue

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
    if [[ ${#in_backticks} -gt 4 ]]; then
      underscored=$(echo ${in_backticks} | tr '-' '_')
      if [ ${underscored} != ${in_backticks} ]; then
        pattern="(${in_backticks}|${underscored})"
      else
        pattern="${in_backticks}"
      fi
      echo "Pattern: ${pattern}"
      # Find the reference, if any, and create the links, separated by spaces.
      links=$(rg -INio --null-data '\[(c-)?([^\]]*'${pattern}'[^-]*)\]:' -r '[`'"${in_backticks}"'`][$1$2]⮳{{hi:'"${in_backticks}"'}}' ${root}src/refs/crate-refs.md | tr '\0' ' ')
      if [ -n "${links}" ]; then
        links=$( sed -E -e 's#(\\|~|&)#\\\1#g' <<< "$links" ) # Escape \ ~ & and newlines
        echo "Links: ${links}"
        # Replace `<word>` by the links, but only if the line does not start with # (heading)
        sed -E -i 's~^([^#].*[^\[`])?`'"${in_backticks}"'`~\1'"${links}"'~g' "${file}" # -n   p
      else
        echo "[${in_backticks}]: " | sed -E -e 's/<T>//g' >> ${root}suggest.md
      fi
    fi
  done
done

sort -u -o ${root}suggest.md ${root}suggest.md
