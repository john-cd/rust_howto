set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list chapters --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CHAPTER MANAGEMENT -----------------------------------

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
[unix]
hide:
  ./hide.sh

# Make all markdown sections / pages visible
[confirm]
[unix]
unhide:
  ./unhide.sh
