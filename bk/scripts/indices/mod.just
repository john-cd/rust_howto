set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- INDICES MANAGEMENT -----------------------------------

## ---- CRATE INDICES -----------------------------------

help := 'help'
empty := ''

# Create the `crates by category` and `crates (alphabetical)` pages
[no-cd]
@crate_indices cmd=help *subcmd=empty:
  {{root}}/../bin/crate-indices {{cmd}} {{subcmd}}
# or: cargo run -p crate_indices -- {{cmd}} {{subcmd}}

## ---- INDEX OF EXAMPLES --------------------------------

# Outputs the contents of index of examples `src/examples_index.md` (reading the local tables of content of all subchapters)
[group('index of examples')]
[unix]
generate_index_of_examples:
  ./generate_index_of_examples.sh {{root}}
# Usage: just generate_index_of_examples

# Add, to `src/refs.incl.md`, missing references that are required for the index of examples (found in `examples_index.md`)
[group('index of examples')]
[unix]
update_refdefs_for_examples:
  ./update_refdefs_for_index_of_examples.sh {{root}}
# Usage: just indices update_refdefs_for_index_of_examples

## ---- OTHERS --------------------------------

# Quick and dirty generation of language/index.md; manual editing required
[unix]
generate_language_index:
  ./generate_language_index.sh {{root}}
# Usage: just indices generate_language_index >> ./src/language/index.incl.md

# Generate the summary references for the dev tool category
[unix]
generate_dev_tools_refdefs:
  #!/usr/bin/env bash
  set -euo pipefail
  root="{{root}}/src/categories/development-tools/"
  mv -f ${root}refs.incl.md ${root}refs.incl.md.bak
  for file in $(find ${root} -mindepth 2 -type f -name "refs.incl.md")
  do
    rel=$(realpath --relative-to=${root} $file)
    rel=$(dirname $rel)'/'
    sed -E 's~^(\[.*\]:\s*?)(.*)$~\1'${rel}'\2~g' $file >> ${root}refs.incl.md
  done
  sort -u -o ${root}refs.incl.md ${root}refs.incl.md
