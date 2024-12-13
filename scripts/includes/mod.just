_default:
  @just --list includes --unsorted --justfile {{justfile()}}
#@just --choose

## ---- INCLUDE MANAGEMENT -----------------------------------

# Make sure that the local references i.e. {{#include refs.incl.md}} are included in every markdown file; note that a few files (indices, TOC...) don't need local references
list_missing_local_refs:
  ./list_missing_local_ref_includes.sh

# Make sure that a local recipe table / TOC i.e. {{#include <subchapter>.incl.md}} is present in each subchapter
list_missing_tables:
  ./list_missing_subchapter_includes.sh
