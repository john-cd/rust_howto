# Purpose: Find Markdown pages missing from the book table of contents.
# Usage: From bk/, run just toc <recipe>.
#
# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in `SUMMARY.md`
@list_missing_chapters:
    ./list_missing_chapters_in_toc.sh {{root}}

# Usage: just list_missing_chapters_in_toc >> ./src/SUMMARY.md
