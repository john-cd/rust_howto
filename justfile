# Purpose: Coordinate validation and maintenance across the Rust book workspaces.
# Usage: Run just --list from this directory to inspect the available recipes.

# TODO Reviewed ??-??-2026.

import "scripts/build.just"

set unstable

set shell := ["bash", "-uc"]

# Use Git Bash on Windows so recipes share one shell syntax across platforms.
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

# Format Cargo manifests in every independent Rust workspace.
@fmt-toml:
    set -e; for workspace in bk/crates bk/crates/cats/memory_management/allocator_selection bk/scripts/book/testdata/refdefs/crates bk/crates/cats/database/examples/query_builders_orms/toasty_validation later/crates later/crates/cats/gui/vizia later/crates/cats/development_tools_ffi/python_extension later/crates/cats/embedded later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest tools xmpl playground publish; do \
      echo "Formatting Cargo manifests in ${workspace}"; \
      cargo fmt-toml --workspace-path "${workspace}"; \
    done

# Check Cargo manifest formatting in every independent Rust workspace.
@fmt-toml-check:
    set -e; for workspace in bk/crates bk/crates/cats/memory_management/allocator_selection bk/scripts/book/testdata/refdefs/crates bk/crates/cats/database/examples/query_builders_orms/toasty_validation later/crates later/crates/cats/gui/vizia later/crates/cats/development_tools_ffi/python_extension later/crates/cats/embedded later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest tools xmpl playground publish; do \
      echo "Checking Cargo manifests in ${workspace}"; \
      cargo fmt-toml --check --workspace-path "${workspace}"; \
    done

# Check dependency advisories, licenses, duplicate versions, and registry sources.
@deny:
    set -e; for workspace in bk/crates bk/crates/cats/memory_management/allocator_selection bk/scripts/book/testdata/refdefs/crates bk/crates/cats/database/examples/query_builders_orms/toasty_validation later/crates later/crates/cats/gui/vizia later/crates/cats/development_tools_ffi/python_extension later/crates/cats/embedded later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest tools xmpl playground publish; do \
      echo "Checking dependency policy in ${workspace}"; \
      cargo deny --locked --manifest-path "${workspace}/Cargo.toml" --workspace check; \
    done

# Run a recipe in every workspace and report failure if any workspace fails.
@all cmd:
    status=0; for wkspace in bk later mdbook-utils playground publish tools xmpl; do \
      if [ "$wkspace" = "mdbook-utils" ] && [ ! -f "$wkspace/justfile" ]; then \
        echo "Skipping optional workspace ${wkspace} (submodule not initialized)"; \
        continue; \
      fi; \
      just "$wkspace" {{ quote(cmd) }} || status=1; \
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

# Install the repository Git hook shim using prek.
hooks-install:
    prek install

# Run prek against the staged files.
hooks-run:
    prek run
