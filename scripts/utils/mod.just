set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list utils --unsorted --justfile {{justfile()}}
#@just --choose

## ---- UTILITIES -----------------------------------

## .... SPELL ....

# Check spelling in markdown
[unix]
spell:
  #! /bin/bash
  root={{justfile_directory()}}/
  ${root}.devcontainer/spellcheck.sh list

[windows]
spell:
  @echo "Spell check is not implemented for Windows!"

## .... GET LINK ....

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
[unix]
make_link pattern:
  ./make_link.sh {{pattern}}

[windows]
make_link pattern:
  @echo "Not implemented for Windows!"

## ....  ....

help := 'help'
empty := ''

# Call mdbook-utils to manage links, ref definitions, etc...
utils cmd=help *subcmd=empty:
  mdbook-utils {{cmd}} {{subcmd}}
# TODO P2 clarify

# Run the templating tool e.g to create badges and reference definitions for a given crate or category
templ cmd=help *subcmd=empty:
  cargo run -p templ -- {{cmd}} {{subcmd}}
# TODO P2 clarify

# Autogenerate a chapter (from template)
autogen:
  cargo run -p autogen
# TODO P2 finish
