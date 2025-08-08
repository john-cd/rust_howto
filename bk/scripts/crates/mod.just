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
     find {{book_root}}/src {{book_root}}/drafts {{book_root}}/later -type f -name '*.md' -not -name '*.incl.md' -not -name '*refs.md' \
        -exec sed -E -i 's~(\s)('"${crate}"')(\s)~\1`\2`\3~g' {} \;
   done

# Check the master list of crates and print invalid names (slow).
check:
    #! /usr/bin/env bash
    set -uo pipefail
    for crate in $( cat {{book_root}}/master/crates.txt )
    do
      cargo info ${crate} > /dev/null 2>&1 || echo "$crate"
    done
    echo "DONE"

fix:
    #! /usr/bin/env bash
    set -euo pipefail
    root={{book_root}}/
    # Extract crate name from labels [c~<name>::optional::path~suffix] in all Markdown files (except refdef files).
    regex='(\[c~)([^:~]+)([^]]+\])'
    # ${root}drafts ${root}later
    #  -g "!*-refs.md"
    rg --no-line-number --no-filename --only-matching -g "*.md" --replace '$2' "${regex}" "${root}src" ${root}drafts ${root}later | sort -u | comm --check-order -23 - <(sort -u ${root}master/crates.txt)

repl:
    #! /usr/bin/env bash
    set -euo pipefail
    root={{book_root}}/
    crates=$(grep '_' ${root}/temp.txt)
    for crate in $crates
    do
      new_crate=$(sed 's/_/-/g' <<< "$crate" )
      echo ">>> ${new_crate}"
      find "${root}" -type f -name "*.md" -exec sed -E -i 's/'"${crate}"'/'"${new_crate}"'/g' {} \;
    done
