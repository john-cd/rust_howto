#!/usr/bin/env bash
set -euo pipefail

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links

root="$(realpath $1)/"
pattern="$2"
# Look for [c~...pattern...~docs] or [...pattern...] in the global reference definitions
# excluding suffixes like ~repo, ~badge, etc...
rg -INi  '\[(c~)?([^]]*'${pattern}'[^~]*)(~docs)?\]:\s?(.*)' \
   -r'[`$2`][$1$2~docs]↗{{hi:$2}}' ${root}src/refs
#  [![$2][$1$2~docs~badge]][$1$2~docs]  [$1$2~docs]: $4
#  -N = --no-line-number; -I = --no-filename -r = replace -i = ignore case
# To only avoid matching lines that contain ~badge use (?:(?!~badge)[^]])* with the --pcre2 flag

echo "DONE"
