# Purpose: Check local table-of-contents and reference includes in book Markdown.
# Usage: From bk/, run just includes <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- INCLUDE MANAGEMENT -----------------------------------

# Make sure that the local references i.e. {{#include refs.incl.md}} are included in every markdown file; note that a few files (indices, TOC...) don't need local references
@list_missing_local_refs:
    cargo run --locked --manifest-path "{{ root }}/../tools/Cargo.toml" -p example-integrity -- --repo-root "{{ root }}/.." includes local-refs --book-root "{{ root }}"

# Make sure that a local recipe table / TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
@list_missing_tables:
    cargo run --locked --manifest-path "{{ root }}/../tools/Cargo.toml" -p example-integrity -- --repo-root "{{ root }}/.." includes subchapter-tables --book-root "{{ root }}"
