#! /bin/bash

set -euo pipefail

# Simple pre-processor for mdBook
#
# Removes <div class="hidden"></div> sections from the markdown before passing it to the renderer(s)
# Removes includes for hidden files (that start with _)

# The first time it runs the preprocessor to determine if it supports the given renderer.
# mdBook passes two arguments to the process: the first argument is the string supports and the second argument is the renderer name. The preprocessor should exit with a status code 0 if it supports the given renderer, or return a non-zero exit code if it does not.
if [ "${1-invalid}" == "supports" ]; then
    #echo "INFO: called with: $1 $2" > purge.log
    exit 0
fi

# If the preprocessor supports the renderer, then mdbook runs it a second time, passing JSON data into stdin. The JSON consists of an array of [context, book]
# where context is the serialized object PreprocessorContext and book is a Book object containing the content of the book.
# The preprocessor should return the JSON format of the Book object to stdout, with any modifications it wishes to perform.

#echo "INFO: called without args" >> purge.log

# Save to tempfile first
tempfile=$(mktemp)
cat < /dev/stdin > $tempfile
#echo "INFO: saved stdin in: cat $tempfile" >> purge.log

# We also could do:
#local input="$(< /dev/stdin)"

# Extract the book object from [context, book]
# Remove includes that points to hidden files
# Remove the hidden sections between <div class="hidden"> and </div>
jq '.[1]' $tempfile | sed -E -e 's~\{\{#include _[^{}]+?\.incl\.md\}\}~~g' -e 's~(<div class=\\"hidden\\">)[^<]+?(</div>)~\1REMOVED\2~g'
# debug: | tee -a purge.log
#echo "INFO: processed $tempfile" >> purge.log

## Debug:

# Context:
# first=$(jq '.[0]' $tempfile)
# #echo "$first" >> purge.log

# Content only
# second=$(jq '.[1].sections[].Chapter?.content?' $tempfile)
# echo "$second" >> purge.log
