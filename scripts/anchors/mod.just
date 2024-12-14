set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list anchors --unsorted --justfile {{justfile()}}
#@just --choose

## ---- ANCHOR MANAGEMENT -----------------------------------

# List book headings that do not have an anchor e.g. {#some-text}. Note that not all headers need one.
[unix]
list_missing:
  ./list_missing_anchors.sh

# (BEWARE: modifies files directly) Generate reference definitions from heading anchors e.g. {#some-text} and add them to `refs.incl.md`
[unix]
generate_refdefs_from:
  ./generate_refdefs_from_anchors.sh
