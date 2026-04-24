set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- RECIPE TABLE MANAGEMENT -----------------------------------

# (BEWARE: modifies files directly) Add links to recipes to `<subchapter>.incl.md` files, using the local reference definitions in `refs.incl.md`. Also makes the titles in the recipe tables match the corresponding headings of the subchapters.
[confirm]
[unix]
@fix_recipe_tables:
  ./fix_recipe_tables.sh {{root}}
