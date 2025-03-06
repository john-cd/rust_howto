set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CRATE MANAGEMENT -----------------------------------

# (BEWARE: modifies many files; manual review required) put all crate names in the book's Markdown between ` ` (the crate names are sourced from a text file passed as an argument)
backtick crate_list:
   #! /usr/bin/env bash
   set -euo pipefail
   for crate in $( cat {{root}}/{{crate_list}} | sed -e '/^(time|just|directories|image|open|log|either|which|similar|cargo|just)$/d' )
   do
     echo ">> $crate"
     find {{root}} -type f -name '*.md' -not -name '*.incl.md' -not -name '*refs.md' \
        -exec sed -E -i 's~(\s)('"${crate}"')(\s)~\1`\2`\3~g' {} \;
   done
# just crates backtick ../tools/crate_indices/crates.txt
