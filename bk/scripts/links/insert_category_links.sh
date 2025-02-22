#!/usr/bin/env bash
set -u

# Insert links to categories: [<category>][cat-<category>]
#
# Usage <script>.sh <root folder of book>
# Beware: may modify a lot of files. Manual editing is required after run.
# Does not handle text in ``` ``` blocks correctly.

root="$(realpath $1)/"

# Lines that doe not start with { # [ and character before is not a letter or { # [ -
regex='(^[^{#[].*[^a-zA-Z/{#:`\[-])'

# Markdown files
files=$(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "word_index.md" -not -name "*.incl.md" -not -wholename "crates/*" -not -wholename "refs/*" \))

## Extract categories from refdefs e.g. [cat-accessibility]: https://crates.io/categories/accessibility
for cat in $(rg -INio 'https://crates.io/categories/(.+)' -r '$1' ${root}src/refs/category-refs.md)
do
    echo "Category: ${cat}"
    if [[ ${#cat} -lt 3 ]]; then
      echo "Skip"
      continue
    fi
    with_space=$(echo "${cat}" | sed -E 's/([^:]+)::([^:]+)/\1 \2|\2/g; s/-/ /g')
    if [ "${with_space}" != "${cat}" ]; then
      pattern="(${cat}|${with_space})"
    else
      pattern="(${cat})"
    fi
    echo "Pattern: ${pattern}"
    # Insert potential links to crates into the book's Markdown
    for file in ${files}
    do
      # Insert [...][cat-<category>]⮳{{hi:<category>}}
      sed -E -i 's~'"${regex}${pattern}"'([^a-zA-Z._-])~\1[\2][cat-'"${cat}"']⮳{{hi:'"${cat}"'}}\3~Ig' "${file}"
      if [ $? -ne 0 ]; then
        echo "Error: ${file}"
      fi
    done
done

echo "DONE"
