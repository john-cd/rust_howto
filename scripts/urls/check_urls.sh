#! /bin/bash
set -euo pipefail

# Check that URLs to external websites e.g. https://... (typically found in ref defs) are valid and working (e.g. no 404).
# This script does NOT check whether reference definitions are used or not.
#
# Usage: /code/scripts/urls/check_urls.sh

root="/code/"
lychee --exclude-all-private --no-ignore --hidden --format detailed --cache "${root}**/*.md" "${root}**/*.toml" "${root}**/*.yaml" "${root}**/*.yml"
# We could also check "${root}.devcontainer/*" "${root}**/*.sh"

# Somehow lychee ignores links in markdown reference definitions... thus the use of `sed` to extract URLs first
sed -r 's/\[.+?\]: (.+)$/\1/' ${root}src/refs/*.md | lychee --exclude-all-private --format=detailed --cache -u "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36" -- -

echo "DONE"
