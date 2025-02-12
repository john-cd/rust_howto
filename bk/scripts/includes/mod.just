set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- INCLUDE MANAGEMENT -----------------------------------

# Make sure that the local references i.e. {{#include refs.incl.md}} are included in every markdown file; note that a few files (indices, TOC...) don't need local references
[unix]
list_missing_local_refs:
  ./list_missing_local_ref_includes.sh {{root}}

# Make sure that a local recipe table / TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
[unix]
list_missing_tables:
  ./list_missing_subchapter_includes.sh {{root}}
