set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

book_root := clean(source_directory() + "/../..")

set tempdir := "."

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CRATE MANAGEMENT -----------------------------------

# (BEWARE: modifies many files; manual review required) put all crate names in the book's Markdown between ` `
backtick:
   #! /usr/bin/env bash
   set -euo pipefail
   for crate in $( cat {{book_root}}/master/crates.txt | sed -E -e '/^\s*(protobuf|ignore|select|windows|tracing|heapless|bytes|regex|futures|cached|config|diff|which|fake|glob|markdown|insta|similar|time|just|directories|image|open|log|either|which|similar|cargo|just|cross|colored|console)\s*$/d' )
   do
     echo ">> $crate"
     find {{book_root}}/src {{book_root}}/drafts  {{book_root}}/later -type f -name '*.md' -not -name '*.incl.md' -not -name '*refs.md' \
        -exec sed -E -i 's~(\s)('"${crate}"')(\s)~\1`\2`\3~g' {} \;
   done
