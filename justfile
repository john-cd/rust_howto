# TODO Reviewed ??-??-2026

import "scripts/build.just"

set unstable

set shell := ["bash", "-uc"]

# Use Git Bash on Windows so recipes share one shell syntax across platforms.
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

# Run a recipe in every workspace and report failure if any workspace fails.
@all cmd:
  status=0; for wkspace in bk later mdbook-utils playground publish tools xmpl; do \
    just "$wkspace" {{quote(cmd)}} || status=1; \
    echo "------------------------"; \
  done; exit "$status"

# Book
mod bk
# Later
mod later
# mdBook utilities
mod? mdbook-utils
# Playground
mod playground
# Placeholder crate on `crates.io`
mod publish
# Tools
mod tools
# Additional examples
mod xmpl
