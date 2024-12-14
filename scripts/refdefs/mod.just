set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list refdefs --unsorted --justfile {{justfile()}}
#@just --choose

## ---- REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
[unix]
sort:
  ./sort_refdefs.sh

[windows]
sort:
  echo "`sortrefs` is not implemented while working in Windows!"

# List links without corresponding reference definitions and vice versa
[unix]
check:
  ./check_refdefs.sh
