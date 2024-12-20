set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list deps --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CRATE MANAGEMENT -----------------------------------

# Update Cargo.lock dependencies for all projects (incl. the book's examples, tools, and additional examples in `crates/xmpl`)
[confirm]
update:
  cargo update

# Get info about a crate (Rust 1.82+)
info crate:
  cargo info {{crate}}

# Check for security advisories, license issues, etc
check_crates:
  cd {{justfile_directory()}} && cargo deny check --hide-inclusion-graph
# WIP
