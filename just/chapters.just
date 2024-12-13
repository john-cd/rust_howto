_default:
  @just --list chapters --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CHAPTER MANAGEMENT -----------------------------------

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
hide:
  {{justfile_directory()}}/scripts/hide/hide.sh

# Make all markdown sections / pages visible
[confirm]
unhide:
  {{justfile_directory()}}/scripts/hide/unhide.sh
