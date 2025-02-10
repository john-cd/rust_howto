 #!/usr/bin/env bash
set -euo pipefail

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links

root="$(realpath $1)/"
pattern="$2"
# Look for [c-...pattern...] or [...pattern...] in the global reference definitions
# excluding suffixes like -github, -badge, etc... (which start with a dash)
rg -INi  '\[(c-)?([^]]*'${pattern}'[^-]*)\]:\s?(.*)' \
   -r'[`$2`][$1$2]⮳  [![$2][$1$2-badge]][$1$2]  [$1$2]: $3' ${root}src/refs
#  -N = --no-line-number; -I = --no-filename -r = replace -i = ignore case
# To only avoid matching lines that contain -badge use (?:(?!-badge)[^]])* with the --pcre2 flag

echo "DONE"
