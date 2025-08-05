#!/usr/bin/env bash
set -euo pipefail

# WORK IN PROGRESS
# - Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
# - Process http://... naked links

# Examples of URLs to process:
# https://arangodb.com/
# https://crates.io/crates/sd
# https://github.com/rustdesk/rustdesk
# Examples of URLs to skip:
# https://github.com/john-cd/rust_howto/issues/1375


# [fix](https://github.com/john-cd/rust_howto/issues/1238)
# 1. do not convert links to GitHub issues e.g. https://github.com/john-cd/rust_howto/issues/1375 links
# 2. skip text between ``` and ```
# p = print; t = branch; : = label
# '/```/, /```/n; tx;     ; :x ;' "${file}"
# 3. move refdefs into central file
# 4. respect conventions

root="$(realpath $1)/"

for file in $( find ${root}src ${root}drafts ${root}later -type f -name "*.md"  -not -name "refs.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
do
  echo ">> $file"
  # Replace naked URLs by reference-style link.
  #
  # -n = quiet -E = extended regex -s = separate files
  # \=...= instead of /.../
  # Trick: '\'' to escape ' in bash
  # for lines that start with http... or for http... not preceded by " ' or (
  sed -n -E -s '\=(^|[^"'\''(])https?://= {
  # replace https::/github.com/.../...
  s=([^"'\''(]?https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'\'')↗]*)?=[`\3`][\3~github] [\3~github]: \1\2\3\4=gp;
  # skip to end if the above matched
  tx;
  # General case http://...
  s=([^"'\''(]?https?://)([^/\s]+)(/[^"'\'')↗]*)?=[\2][\2] [\2]: \1\2\3=gp ;
  :x
  }
  ' "${file}"

  # Replace [...](...)
  #sed -E -n 's=\[(`)?([^`]+?)(`)?\]\((.+?)\)=[\1\2\3][\L\2] \n[\2]: \4\n=gp' "${file}" # use -i to replace in-place
done

echo "DONE"
