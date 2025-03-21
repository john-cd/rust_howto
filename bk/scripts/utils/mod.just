set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- UTILITIES -----------------------------------

## .... SPELL ....

# Check spelling in markdown
[unix]
@spell extra_param='':
  {{root}}/../.devcontainer/spellcheck.sh {{extra_param}}

[windows]
@spell extra_param='':
  echo "Spell check is not implemented for Windows!"

## .... TOOLS ....

help := 'help'
empty := ''

# Call mdbook-utils to manage links, ref definitions, etc...
@utils cmd=help *subcmd=empty:
  cd {{root}} && mdbook-utils {{cmd}} {{subcmd}}
# [clarify](https://github.com/john-cd/rust_howto/issues/1254)

# Run the templating tool e.g to create badges and reference definitions for a given crate or category
[unix]
@templ cmd=help *subcmd=empty:
  if [ ! -f {{root}}/../bin/templ ]; then just -f {{root}}/../tools/justfile release; fi
  cd {{root}} && ../bin/templ {{cmd}} {{subcmd}}
# [clarify](https://github.com/john-cd/rust_howto/issues/1254)

[windows]
@templ cmd=help *subcmd=empty:
  if ( -Not (Test-Path -Path {{root}}\..\bin\templ.exe )) { just -f {{root}}\..\tools\justfile release }
  cd root; {{root}}\..\bin\templ.exe {{cmd}} {{subcmd}}

# Autogenerate a chapter (from template)
#@autogen:
#  {{root}}/../bin/autogen
# [finish](https://github.com/john-cd/rust_howto/issues/1256)
