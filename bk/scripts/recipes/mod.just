# Purpose: Update recipe tables and synchronize their titles with book headings.
# Usage: From bk/, run just recipes <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- RECIPE TABLE MANAGEMENT -----------------------------------

# (BEWARE: modifies files directly) Add links to recipes to `<subchapter>.incl.md` files, using local reference definitions. Also sync table titles with the subchapter headings.
[confirm]
@fix_recipe_tables:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p recipe-tables -- "{{ root }}"
