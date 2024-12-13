_default:
  #@just --choose
  @just --list toc --unsorted --justfile {{justfile()}}

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in `SUMMARY.md`
list_missing_chapters_in_toc:
  {{justfile_directory()}}/scripts/main_table_of_contents/list_missing_chapters_in_toc.sh
# Usage: just list_missing_chapters_in_toc >> ./src/SUMMARY.md
