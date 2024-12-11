#! /bin/bash

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...

for file in $(find . -type f \( -name "*.md" -not -name "SUMMARY.md" \) )
do
  #echo ">> $file"
  sed -E -i 's~\[(`)?([^`]+?)(`)?\]\((.+?)\)~[\1\2\3][\L\2] \n[\2]: \4\n~g' $file
done

echo "DONE"
