# Purpose: Find and convert wiki-style links in book pages.
# Usage: From bk/, run just wikilinks <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- WIKILINK MANAGEMENT -----------------------------------

# Get Wikilinks for chapters (pages) containing a given keyword
@for keyword:
    ./list_chapters.sh {{ root }} {{ keyword }}

# List all chapters in Wikilink format
@list:
    ./list_chapters.sh {{ root }}

# (BEWARE: modifies many files) Convert wiki-style links [[...|...]] to [...][p~...] and [p~...]: ...
@dewikilink:
    ./dewikilink.sh {{ root }}
