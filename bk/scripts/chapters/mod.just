set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CHAPTER MANAGEMENT -----------------------------------

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
