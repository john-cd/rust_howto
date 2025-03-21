set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CRATE MANAGEMENT -----------------------------------

# Update Cargo.lock dependencies for all projects (incl. the book's examples, tools, and additional examples in `crates/xmpl`)
[confirm]
@update:
  cargo update

# Get info about a crate (Rust 1.82+)
@info crate:
  cargo info {{crate}}

# Check for security advisories, license issues, etc
@check_crates:
  cd {{root}} && cargo deny check --hide-inclusion-graph
# TODO
