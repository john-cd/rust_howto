#! /bin/bash
set -euo pipefail

# Create reference-style links and reference definitions, which can be used to replace bare URLs in the Markdown
# Manual review necessary
# Requires ripgrep
#
# Usage: /code/scripts/urls/list_bare_urls.sh

root="/code/"
# Look for http(s)://... and outputs references
rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ${root}src \
-g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[$2-website]: $1$2$3' | sort | sed 's~/$~~'

# Outputs reference-style links
rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ${root}src \
-g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[`$2`][$2-website]' | sort | sed 's~/$~~'
# --pcre2 = Perl regex enabled (allows look-arounds) -g = glob, -r = replace

echo "DONE"

# TODO P1 improve; output [...-github] if https://github.com/...
# TODO P1 ignore base URLs within ```...```
