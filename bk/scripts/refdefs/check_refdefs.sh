 #!/usr/bin/env bash
set -euo pipefail

# List links without corresponding reference definitions and vice versa
# Usage: ./scripts/refdefs/check_refdefs.sh <root folder>

root="$(realpath $1)/"
# Extract reference definitions e.g. [label]: http://xyz
grep -Proh '\[[^\[\]]+?\](?=:)' ${root}src ${root}drafts | sort -u > /tmp/defined_refdefs.txt
# grep -r = recursive, h = no-filename, P = perl regex, o = only-matching

# Extract labels preceded by ] e.g. [some_text][label]
grep -Proh '(?<=\])\[[^ \[\]]+?\]' ${root}src ${root}drafts | sort -u > /tmp/used_refdefs.txt

echo ">>> Links w/o reference definition:"
comm -13 --check-order --output-delimiter="|" /tmp/defined_refdefs.txt /tmp/used_refdefs.txt | sort

echo ">>> Reference definitions not used in links:"
comm -23 --check-order --output-delimiter="|" /tmp/defined_refdefs.txt /tmp/used_refdefs.txt | sort

# Counts
echo
echo "Count of reference definitions without links and vice versa:" $(comm -3 --check-order --output-delimiter="|" /tmp/defined_refdefs.txt /tmp/used_refdefs.txt  | wc -l)
echo "Count of reference definitions defined in the refs folder:" $(cat  /tmp/defined_refdefs.txt | wc -l)
echo "Count of reference definitions used in the markdown:" $(cat  /tmp/used_refdefs.txt | wc -l)

echo "DONE"
