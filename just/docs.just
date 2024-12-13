_default:
  @just --list docs --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CODE DOCUMENTATION -----------------------------------

# Build and display the `cargo doc` documentation for a specific package (e.g. deps)
[unix]
doc pkg:
  cargo clean --doc
  cargo doc --no-deps --document-private-items --locked --package {{pkg}}
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000

_builddocall:
  cargo clean --doc
  cargo doc --no-deps --workspace --locked
# optional: --bins --examples

# Build and display the `cargo doc` documentation for all packages
[unix]
docall: _builddocall
  # cargo doc --open does not seem to work when running from a Dev Container in VS Code;
  # the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML.
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000
  # We could also use `live server` for dynamic reloading.
  # See README.md for other alternatives, such as:
  # xdg-open /cargo-target-rust_howto/target/doc/deps/index.html
