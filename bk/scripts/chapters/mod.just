# Purpose: Hide and restore book pages listed in hiddenfiles.txt.
# Usage: From bk/, run just chapters <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- CHAPTER MANAGEMENT -----------------------------------

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
@hide:
    ./hide.sh {{ root }}

# Make all markdown sections / pages visible
[confirm]
@unhide:
    ./unhide.sh {{ root }}
