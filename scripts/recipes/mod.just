set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list recipes --unsorted --justfile {{justfile()}}
#@just --choose

## ---- RECIPE TABLE MANAGEMENT -----------------------------------

# (BEWARE: modifies files directly) Add links to recipes to `<subchapter>.incl.md` files, using the local reference definitions in `refs.incl.md`
[confirm]
[unix]
fix_recipe_tables:
  ./fix_recipe_tables.sh

# Make the titles in the recipe tables / local TOCs match the corresponding headings of the subchapters
[unix]
fix_recipe_titles:
  ./fix_recipe_titles.sh
