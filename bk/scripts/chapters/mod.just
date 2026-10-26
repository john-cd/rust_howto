# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- CHAPTER MANAGEMENT -----------------------------------

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
[unix]
@hide:
  ./hide.sh {{root}}

[windows]
[confirm]
@hide:
  ./hide.sh {{root}}

# Make all markdown sections / pages visible
[confirm]
[unix]
@unhide:
  ./unhide.sh {{root}}

[windows]
[confirm]
@unhide:
  ./unhide.sh {{root}}
