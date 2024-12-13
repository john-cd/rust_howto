_default:
  @just --list refdefs --unsorted --justfile {{justfile()}}
#@just --choose

## ---- REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
[unix]
sortrefs:
  {{justfile_directory()}}/scripts/refdefs/sort_refdefs.sh

[windows]
sortrefs:
  echo "`sortrefs` is not implemented while working in Windows!"

# List links without corresponding reference definitions and vice versa
check_refdefs:
  {{justfile_directory()}}/scripts/refdefs/check_refdefs.sh
