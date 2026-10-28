# Purpose: Build and preview Rust API documentation for the book examples.
# Usage: From bk/, run just docs <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

book_root := clean(source_directory() + "/../..")

## ---- CODE DOCUMENTATION -----------------------------------

# Build and display the `cargo doc` documentation for a specific package
[unix]
@doc pkg:
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{ book_root }}/crates/
  cargo clean --doc
  cargo doc --no-deps --document-private-items --locked --bins --examples --package {{pkg}}
  cd {{ book_root }}/../target/bk/doc/ ; python3 -m http.server 9000

# Build all package documentation; serving it requires a Unix shell.
[unix]
@_builddocall:
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{ book_root }}/crates/
  cargo clean --doc
  cargo doc --no-deps --document-private-items --locked --bins --examples --workspace
# optional:

# Build and display the `cargo doc` documentation for all packages
[unix]
@docall: _builddocall
  cd {{ book_root }}/../target/bk/doc/ ; python3 -m http.server 9000

[windows]
@doc pkg:
  echo "Documentation preview is supported on Unix only." >&2
  exit 1

[windows]
@docall:
  echo "Documentation preview is supported on Unix only." >&2
  exit 1
# cargo doc --open does not seem to work when running from a Dev Container in VS Code;
# the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML.
# We could also use `live server` for dynamic reloading.
# Alternative:
# xdg-open {{ book_root }}/../target/bk/doc/
