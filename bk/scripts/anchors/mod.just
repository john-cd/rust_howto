# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- ANCHOR MANAGEMENT -----------------------------------

# List book headings that do not have an anchor e.g. {#some-text}. Note that not all headers need one.
[unix]
@list_missing:
  ./list_missing_anchors.sh {{root}}

[windows]
@list_missing:
  ./list_missing_anchors.sh {{root}}

# (BEWARE: modifies files directly) Generate reference definitions from heading anchors e.g. {#some-text} and add them to `refs.incl.md`
[unix]
@generate_refdefs_from:
  ./generate_refdefs_from_anchors.sh {{root}}

[windows]
@generate_refdefs_from:
  ./generate_refdefs_from_anchors.sh {{root}}
