_default:
  @just --list anchors --unsorted --justfile {{justfile()}}
#@just --choose

## ---- ANCHOR MANAGEMENT -----------------------------------

# List book headings that do not have an anchor e.g. {#some-text}. Note that not all headers need one.
list_missing_anchors:
  {{justfile_directory()}}/scripts/anchors/list_missing_anchors.sh

# (BEWARE: modifies files directly) Generate reference definitions from heading anchors e.g. {#some-text} and add them to `refs.incl.md`
generate_refdefs_from_anchors:
  {{justfile_directory()}}/scripts/anchors/generate_refdefs_from_anchors.sh
