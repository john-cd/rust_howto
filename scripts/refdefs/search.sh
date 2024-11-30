#! /bin/bash

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links

pattern=$1

# Look for [c-...pattern...] or [...pattern...] in the global reference definitions
rg -IN '\[(c-)?[^]]*'${pattern}'[^]]*\].*' ./src/refs
#  -N = --no-line-number; -I = --no-filename

# Generate possible links
rg -IN -r'[`$2`][$1$2$3]⮳' '\[(c-)?([^]]*'${pattern}'[^]-]*)([^]]*)\]:\s?(.*)' ./src/refs
#  -r = replace

echo "DONE"
