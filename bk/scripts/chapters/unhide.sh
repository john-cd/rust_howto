#!/usr/bin/env bash
set -euo pipefail

## Make all markdown sections / pages visible

root="$(realpath $1)/"

## Rename <_file.md> to <file.md> within the `src` folder
find ${root}src -type f -name "_*.md" -exec bash -c 'p={}; d=$(dirname $p); f=$(basename $p); mv -n -v -- $d/$f $d/${f/#_/}' \;

## Replace all links / references to <_file.md> by <file.md>
## The `sed` regex matches a markdown filename starting with _, after a / or ( or space
## It uses ~ instead of / as the `sed` separator, in order to keep us sane
find ${root}src -type f -name "*.md" -exec sed -r -i 's~(/|\(|\s)_([a-zA-Z0-9_-]+?\.md)~\1\2~g' {} \;

echo "DONE"
