#! /bin/bash

# List files that do not include their local references i.e. {{#include refs.incl.md}} is missing
# NOTE: a few files (indices, TOC...) don't need local references
#
# Usage: ./scripts/includes/list_missing_local_ref_includes.sh

grep -PrL --exclude=*.incl.md --exclude=*refs.md '\{\{#include refs.incl.md\}\}' ./src
# -P = Perl regex; -r = recursive ; -L = print only names of FILEs with no selected lines

echo "DONE"
