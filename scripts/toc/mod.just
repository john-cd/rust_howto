set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list toc --unsorted --justfile {{justfile()}}
#@just --choose

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in `SUMMARY.md`
[unix]
list_missing_chapters:
  ./list_missing_chapters_in_toc.sh
# Usage: just list_missing_chapters_in_toc >> /code/src/SUMMARY.md
