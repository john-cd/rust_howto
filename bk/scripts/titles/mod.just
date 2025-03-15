set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- TITLE MANAGEMENT -----------------------------------

# (Manual review required) Capitalize titles in the book's Markdown.
normalize:
  #! /usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  root={{root}}
  # Find all Markdown ATX headings.
  for file in $(rg -N -l -t md -g '!*.incl.md' -g'!*refs.md' '^#+\s*.+$' "${root}/src" "${root}/drafts" )
  do
  echo ">> ${file}"
  # Capitalize all words longer than 4 letters, except a few listed below.
  sed -i -E 's/^(#\s*.*?\s+)([a-z]{4,}([ ,;.-]|$))/\1\u\2/g; s/^(#.*\s)(About|With|From|Into|Like|Then|That|Both|Over|Beyond)(\s)/\1\L\2\3/g; s/^(#.*\s)(use)(\s)/\1\u\2\3/g' "${file}"
  done
  echo "DONE"
