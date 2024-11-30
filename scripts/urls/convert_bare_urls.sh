#! /bin/bash

# Create a reference definition for bare URLs in the markdown (manual review necessary)
# Requires ripgrep
# Usage: ./scripts/urls/convert_bare_urls.sh

# Look for http(s)://... and outputs a refe
rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ./src \
-g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[$2-website]: $1$2$3' | sort | sed 's~/$~~'

rg --pcre2 --no-line-number --no-filename --only-matching '(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?' ./src \
-g '*.md' -g '!*refs.md' -g 'refs.incl.md' -r '[`$2`][$2-website]' | sort | sed 's~/$~~'
# --pcre2 = Perl regex enabled (allows look-arounds) -g = glob, -r = replace


# TODO improve; output [...-github] if https://github.com/...
