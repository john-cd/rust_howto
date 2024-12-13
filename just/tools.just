_default:
  #@just --choose
  @just --list tools --unsorted --justfile {{justfile()}}

## ---- UTILITIES -----------------------------------

# Check spelling in markdown
[unix]
spell:
  {{justfile_directory()}}/.devcontainer/spellcheck.sh list

[windows]
spell:
  echo "Spell check is not implemented for Windows!"

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

# Create the `crates by category` and `crates (alphabetical)` pages
crate_indices cmd=help *subcmd=empty:
  cargo run -p crate_indices -- {{cmd}} {{subcmd}}
# TODO P2 clarify

# Autogenerate a chapter (from template)
autogen:
  cargo run -p autogen
# TODO P2 finish
