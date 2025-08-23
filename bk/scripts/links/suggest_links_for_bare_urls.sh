#!/usr/bin/env bash
set -u

# WORK IN PROGRESS
# Create reference-style links and reference definitions, which can be used to replace bare URLs in the Markdown
# Manual review necessary
# Requires ripgrep
#
# Usage: ./scripts/urls/<script_name>.sh <root folder>

root="$(realpath $1)/"

# [pass a var](https://github.com/john-cd/rust_howto/issues/1243)
#pattern=('(?<!: |["`([])(http(?:s)?://(?:www\d?\.|github\.com/)?)([^./]+)(\S+)?')
#"${pattern[@]}"

for file in $( find ${root}src ${root}drafts -type f -name "*.md"  -not -name "refs.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
do
  echo ">> $file"
  contents=$(rg --multiline --invert-match '`.*`' $file)
  # Look for http(s)://... and outputs references
  # Outputs reference-style links
  {
  echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~website] [$2~website]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:www\d?\.)?)([^./]+)(\S+)?'
  echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~repo] [$2~repo]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:github\.com/)?)([^./]+)(\S+)?'
  # --pcre2 = Perl regex enabled (allows look-arounds) -g = glob, -r = replace
  } | sed 's=/$==' | sort
done

echo "DONE"
