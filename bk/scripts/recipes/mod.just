# Purpose: Update recipe tables and synchronize their titles with book headings.
# Usage: From bk/, run just recipes <recipe>.
#
# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- RECIPE TABLE MANAGEMENT -----------------------------------

# (BEWARE: modifies files directly) Add links to recipes to `<subchapter>.incl.md` files, using the local reference definitions in `refs.incl.md`. Also makes the titles in the recipe tables match the corresponding headings of the subchapters.
[confirm]
[unix]
@fix_recipe_tables:
  python3 "{{source_directory()}}/fix_recipe_tables.py" "{{root}}"

[confirm]
[windows]
@fix_recipe_tables:
  python "{{source_directory()}}/fix_recipe_tables.py" "{{root}}"
