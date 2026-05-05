set unstable

set shell := ["bash", "-uc"]

# Set shell for Windows OSs:
# PowerShell Core has some of the Unix shell skills we need, such as || and &&
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

[unix]
all cmd:
  #!/usr/bin/env bash
  set -euo pipefail
  for wkspace in "bk" "later" "mdbook-utils" "playground" "publish" "tools" "xmpl"
  do
    just ${wkspace} {{cmd}} || true
    echo "------------------------"
  done

[windows]
@all cmd:
  $wkspaces = @("bk", "later", "mdbook-utils", "playground", "publish", "tools", "xmpl"); foreach ($wk in $wkspaces) { just $wk {{cmd}} ; Write-Host "------------------------" }

# Book
mod bk
# Later
mod later
# mdBook utilities
mod mdbook-utils
# Playground
mod playground
# Placeholder crate on `crates.io`
mod publish
# Tools
mod tools
# Additional examples
mod xmpl
