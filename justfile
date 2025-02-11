set unstable

set shell := ["bash", "-uc"]
set script-interpreter := ["bash", "-eu"]

# Set shell for Windows OSs:
# PowerShell Core has some of the Unix shell skills we need, such as || and &&
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

[script]
fa:
  set -euo pipefail
  for wkspace in "bk" "mdbook-scrub" "playground" "publish" "tools" "xmpl"
  do
    just $wkspace fa
    echo "------------------------"
  done

# Book
mod bk
# Playground
mod playground
# Placeholder crate on crates.io
mod publish
# `mdbook-scrub` preprocessor
mod? scrub 'mdbook-scrub'
# Tools
mod tools
# Additional examples
mod xmpl
