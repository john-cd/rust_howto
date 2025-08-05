# Manage and Build Code

{{#include code_build.incl.md}}

| Topic | Tools |
|---|---|
| Build System | [cargo][p~cargo] is the primary build system for Rust. |
| Build Scripts | Use `build.rs` files in your project. |
| Build Profiles | Configure build options in `Cargo.toml` for debug, release, etc. |
| Cross-Compilation | [`cross`][c~cross~docs]↗{{hi:cross}} simplifies cross-compilation, target specifications in `Cargo.toml`. |
| Link-Time Optimization (LTO) | Controlled via `Cargo.toml`. |
| Build Dependencies | Managed by [cargo][p~cargo] |
| Incremental Compilation | Handled by [cargo][p~cargo]. |
| Build Automation (for complex builds) | [`xtask`][c~xtask~docs]↗{{hi:xtask}} |
| Compiler Flags | Configurable in `Cargo.toml` |
| Code Generation | Often done with procedural [macros][p~macros] or build scripts. |

## Save and run project-specific Commands with the `just` Command Runner {#just}

[![just][c~just~docs~badge]][c~just~docs] [![just~crates.io][c~just~crates.io~badge]][c~just~crates.io] [![just~github][c~just~github~badge]][c~just~github] [![just~lib.rs][c~just~lib.rs~badge]][c~just~lib.rs]{{hi:just}}{{hi:Command-line}}{{hi:Development}}{{hi:Runner}}{{hi:Task}}{{hi:Utility}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`just`][c~just~docs]↗{{hi:just}}{{hi:just}} is a command runner{{hi:Command runner}}. It is a Rust-based equivalent to `make` without the ability to detect file changes, but with significantly fewer syntactic warts.

Consult the [Just programmer's manual][c~just~programmer-manual]↗.

### Create a `justfile` {#skip1}

Place the following example `justfile` in the root folder of your project. Run [`just`][c~just~website]{{hi:just}} to see a list of recipes. Run `just <recipe>` to execute the corresponding recipe.

```makefile
# Load a .env file, if present.
set dotenv-load

default:
 @just --list --unsorted

# Check a local package and all of its dependencies for errors
check:
 @cargo check

# Compile a local package and all of its dependencies
build: check
 @cargo build

# Run a binary or example of the local package
run: check
 @cargo run

system-info:
 @echo "This is an {{arch()}} machine".

# Shebang script example
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"
```

### Install `just` in a Dev Container {#skip2}

[`just`][c~just~docs]↗{{hi:just}}

```Dockerfile
FROM mcr.microsoft.com/devcontainers/base:bullseye
# Or perhaps `mcr.microsoft.com/devcontainers/rust:bullseye` if you want Rust & `cargo`

SHELL ["bash", "-c"]

# Prerequisites to Install Just: https://just.systems/man/en
RUN <<EOF
 wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
 echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
 sudo apt update
EOF

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install just \
  && apt-get autoremove -y && apt-get clean -y
```

For Alpine, use [`apk`][apk~website]{{hi:apk}}↗:

```dockerfile
RUN apk add just
```

## Check Your Rust Code in the Background {#bacon}

[![bacon][c~bacon~docs~badge]][c~bacon~docs]{{hi:bacon}}
[![bacon~crates.io][c~bacon~crates.io~badge]][c~bacon~crates.io]
[![bacon~github][c~bacon~github~badge]][c~bacon~github]
[![bacon~lib.rs][c~bacon~lib.rs~badge]][c~bacon~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`bacon`][c~bacon~docs]↗{{hi:bacon}} is a background rust code checker. It is designed for minimal interaction, so that you can just let it run, alongside your editor, and be notified of warnings, errors, or test failures in your Rust code.

```sh
# Install or Update `bacon`
cargo install --locked bacon

# Check the current project
bacon

# Run `clippy` instead of `cargo check`
bacon clippy
```

## Related Topics

Consider [`cargo-make`][c~cargo_make~docs]↗{{hi:cargo-make}} if you want something with a bulkier syntax but more cross-platform portability.

See [[building | Building]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[review; dedupe this page and cargo plugins > building; move from other section NOW](https://github.com/john-cd/rust_howto/issues/919)
</div>
