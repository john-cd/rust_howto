#! /bin/bash

# List (sub)chapters that somehow were not added in the main table of contents i.e. SUMMARY.md
# Usage: ./scripts/main_table_of_contents/list_missing_chapters_in_toc.sh

# List all markdown files, except TOCs and references
for file in $(find ./src -type f -name "*.md" -not -name '*.incl.md' -not -name "*refs.md" -not -name "SUMMARY.md")
do
    rel=$(realpath --relative-to=./src $file)
    # Check whether the path to the (sub)chapter is not in SUMMARY.md
    in_toc=$(grep -Poh ${rel} ./src/SUMMARY.md)
    if [ -z "$in_toc" ]; then
        base=$(basename $file | awk 'BEGIN{split("a the to at in on with and but or",w); for(i in w)nocap[w[i]]}function cap(word){return toupper(substr(word,1,1)) tolower(substr(word,2))}{for(i=1;i<=NF;++i){printf "%s%s",(i==1||i==NF||!(tolower($i) in nocap)?cap($i):tolower($i)),(i==NF?"\n":" ")}}')
        echo "- ["${base%.md}"]("$rel")"
    fi
done

echo "DONE"
