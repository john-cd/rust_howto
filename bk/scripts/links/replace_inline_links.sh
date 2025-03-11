#!/usr/bin/env bash
set -euo pipefail

# TODO FIX do not convert links to GitHub issues
# respect conventions
# move refdefs into central file

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...

root="$(realpath $1)/"

for file in $(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" \) )
do
  #echo ">> $file"
  sed -E -i 's~\[(`)?([^`]+?)(`)?\]\((.+?)\)~[\1\2\3][\L\2] \n[\2]: \4\n~g' $file
done

echo "DONE"
