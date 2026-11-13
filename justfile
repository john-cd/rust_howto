# Purpose: Coordinate validation and maintenance across the Rust book workspaces.
# Usage: Run just --list from this directory to inspect the available recipes.

# TODO Reviewed ??-??-2026.

import "scripts/build.just"

set unstable

set shell := ["bash", "-uc"]

# Use Git Bash on Windows so recipes share one shell syntax across platforms.
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]
set quiet

# Uniform task names (short aliases). The aggregate tasks run in every workspace.
alias f := format
alias fa := format-all
alias fc := format-check
alias l := lint
alias lf := lint
alias lint-fix := lint
alias lc := lint-check
alias la := lint-all
alias c := check
alias ca := check-all
alias b := build
alias t := test
alias d := doc
alias de := deny
alias hi := hooks-install
alias hr := hooks-run

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
    set -e; for workspace in bk/crates bk/crates/cats/memory_management/allocator_selection bk/scripts/book/testdata/refdefs/crates bk/crates/cats/database/examples/query_builders_orms/toasty_validation later/crates later/crates/cats/gui/vizia later/crates/cats/development_tools_ffi/python_extension later/crates/cats/embedded later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest tools xmpl playground publish mdbook-utils; do \
      if [ "$workspace" = "mdbook-utils" ] && [ ! -f "$workspace/Cargo.toml" ]; then \
        echo "Skipping optional workspace mdbook-utils (submodule not initialized)"; \
        continue; \
      fi; \
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

# Format Rust code in every workspace.
format: (all "format")

# Format everything in every workspace.
format-all: (all "format-all")

# Check formatting in every workspace.
format-check: (all "format-check")

# Lint with autofix in every workspace.
lint: (all "lint")

# Lint without changes in every workspace.
lint-check: (all "lint-check")

# Lint with autofix, plus extra linters, in every workspace.
lint-all: (all "lint-all")

# Check all targets in every workspace.
check: (all "check")

# Run all checks without modifying files in every workspace (deny is separate).
check-all: (all "check-all")

# Build every workspace.
build: (all "build")

# Test every workspace.
test: (all "test")

# Build documentation in every workspace.
doc: (all "doc")

# Run the local CI gates: check-all plus dependency policy.
ci: check-all deny

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
