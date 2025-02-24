set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CHAPTER MANAGEMENT -----------------------------------

# List, in Wikilink format, chapters (pages) containing a keyword (or list all if none provided).
[unix]
@list keyword='':
  ./list_chapters.sh {{root}} {{keyword}}


# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
[unix]
@hide:
  ./hide.sh {{root}}

# Make all markdown sections / pages visible
[confirm]
[unix]
@unhide:
  ./unhide.sh {{root}}
