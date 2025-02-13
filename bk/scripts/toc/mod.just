set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{source_file()}}

## ---- MAIN TOC MANAGEMENT -----------------------------------

# List (sub)chapters that somehow were not added in `SUMMARY.md`
[unix]
@list_missing_chapters:
    ./list_missing_chapters_in_toc.sh {{root}}
# Usage: just list_missing_chapters_in_toc >> ./src/SUMMARY.md
