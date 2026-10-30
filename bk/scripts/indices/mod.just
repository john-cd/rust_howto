# Purpose: Generate crate, language, and example indexes for the book.
# Usage: From bk/, run just indices <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

exe := if os_family() == "windows" { ".exe" } else { "" }
root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- INDICES MANAGEMENT -----------------------------------
## ---- CRATE INDICES -----------------------------------

help := 'help'
empty := ''

# Create the `crates by category` and `crates (alphabetical)` pages
[no-cd]
@crate_indices cmd=help *subcmd=empty:
    {{ quote(root + "/../bin/crate-indices" + exe) }} {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}

# or: cargo run -p crate_indices -- {{cmd}} {{subcmd}}

# Generate crate page sections for `crates_and_examples.md`, given a list of crate names
[no-cd]
@crate_page *crates=empty:
    {{ quote(root + "/../bin/crate-indices" + exe) }} crate_page {{ crates }}

# Usage: just indices crate_page crate1 crate2 crate3
## ---- INDEX OF EXAMPLES --------------------------------

# Outputs the contents of index of examples `src/examples_index.md` (reading the local tables of content of all subchapters)
[group('index of examples')]
generate_index_of_examples:
    ./generate_index_of_examples.sh {{ root }}

# Usage: just generate_index_of_examples

# Add, to `src/refs.incl.md`, missing references that are required for the index of examples (found in `examples_index.md`)
[group('index of examples')]
update_refdefs_for_examples:
    {{ if os_family() == "windows" { "python" } else { "python3" } }} "{{ source_directory() }}/update_refdefs_for_examples.py" generate --book-root "{{ root }}"

# Usage: just indices update_refdefs_for_examples (cross-platform; `just book refdefs-check` runs the matching check)
## ---- OTHERS --------------------------------

# Quick and dirty generation of language/index.md; manual editing required
generate_language_index:
    ./generate_language_index.sh {{ root }}

# Usage: just indices generate_language_index >> ./src/language/index.incl.md

# Generate the summary references for the dev tool category
generate_dev_tools_refdefs:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{ root }}/src/categories/development-tools/"
    mv -f ${root}refs.incl.md ${root}refs.incl.md.bak
    for file in $(find ${root} -mindepth 2 -type f -name "refs.incl.md")
    do
      rel=$(realpath --relative-to=${root} $file)
      rel=$(dirname $rel)'/'
      sed -E 's~^(\[.*\]:\s*?)(.*)$~\1'${rel}'\2~g' $file >> ${root}refs.incl.md
    done
    sort -u -o ${root}refs.incl.md ${root}refs.incl.md
