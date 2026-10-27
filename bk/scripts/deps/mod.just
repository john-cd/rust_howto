# Purpose: Update and inspect dependencies in the repository Cargo workspaces.
# Usage: From bk/, run just deps <recipe>.
#
# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- CRATE MANAGEMENT -----------------------------------

# Update Cargo.lock dependencies for all projects (incl. the book's examples, tools, and additional examples in `crates/xmpl`)
[confirm]
@update:
    cargo update

# Get info about a crate (Rust 1.82+)
@info crate:
    cargo info {{ crate }}

# Check for security advisories, license issues, etc
@check_crates:
    cd {{ quote(root + "/crates") }} && cargo deny --manifest-path Cargo.toml --workspace --config deny.toml check all --hide-inclusion-graph

# review cargo deny
