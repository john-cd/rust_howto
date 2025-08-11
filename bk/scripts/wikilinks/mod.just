set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- WIKILINK MANAGEMENT -----------------------------------

# List, in Wikilink format, chapters (pages) containing a keyword (or list all if none provided).
[unix]
@list keyword='':
  ./list_chapters.sh {{root}} {{keyword}}


# (BEWARE: modifies many files) Convert wiki-style links [[...|...]] to [...][p~...] and [p~...]: ...
[unix]
@dewikilink:
  ./dewikilink.sh {{root}}
