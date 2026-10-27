<!-- TODO Reviewed ??-??-2026 -->

# Build Rust Code {#build-code}

{{#include building.incl.md}}

| Concern                           | Where to start                                                                                                                                                               |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Build a package                   | Use `cargo check` for fast feedback, `cargo build` to compile, and `cargo run` to build and run a binary.                                                                    |
| Build scripts                     | Add `build.rs` only when a build needs to generate files, inspect the environment, or compile native code. See the [Cargo build-script reference][book~cargo~build-script]↗. |
| Build profiles                    | Set development and release profile options in `Cargo.toml`; measure changes against the workload you care about.                                                            |
| Compiler and linker configuration | Prefer Cargo's target and profile configuration. Keep machine-specific paths out of shared configuration.                                                                    |
| Repeated project tasks            | Use a small `justfile` or a dedicated task runner when ordinary Cargo commands are not enough; see the [Cargo plugin build guide][p~building].                               |

For cross-compilation, linker choices, and compile-time optimization, use the focused [cross-compilation][p~cross-compilation] and [compilation performance][p~performance] guides. This chapter covers Cargo's normal build flow; the separate [Cargo plugin build guide][p~building] covers task runners such as `cargo-make`, `cargo-xtask`, and `xshell`.

## Save and run project-specific Commands with the `just` Command Runner {#just}

[![just][c~just~docs~badge]][c~just~docs] [![just~crates.io][c~just~crates.io~badge]][c~just~crates.io] [![just~repo][c~just~repo~badge]][c~just~repo] [![just~lib.rs][c~just~lib.rs~badge]][c~just~lib.rs]{{hi:just}}{{hi:Command-line}}{{hi:Development}}{{hi:Runner}}{{hi:Task}}{{hi:Utility}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`just`][c~just~docs]↗{{hi:just}}{{hi:just}} is a command runner{{hi:Command runner}}. It is a Rust-based equivalent to [`make`][make~website]↗{{hi:make}} without the ability to detect file changes, but with significantly fewer syntactic warts.

Consult the [Just programmer's manual][c~just~programmer-manual]↗.

### Create a `justfile` {#create-a-justfile .skip}

Place the following example `justfile`{{hi:justfile}} in the root folder of your project. Run [`just`][c~just~website]↗{{hi:just}} to see a list of recipes. Run `just <recipe>` to execute the corresponding recipe.

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

### Install `just` in a Dev Container {#install-just-in-a-dev-container .skip}

[`just`][c~just~docs]↗{{hi:just}}

```Dockerfile
FROM mcr.microsoft.com/devcontainers/base:bullseye
# Or perhaps `mcr.microsoft.com/devcontainers/rust:bullseye` for Rust & `cargo`

SHELL ["bash", "-c"]

# Install Just: <https://just.systems/man/en>
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install just \
  && apt-get autoremove -y && apt-get clean -y
```

For Alpine, use [`apk`][apk~website]↗{{hi:apk}}:

```dockerfile
RUN apk add just
```

## Check Your Rust Code in the Background {#bacon}

[![bacon][c~bacon~docs~badge]][c~bacon~docs]{{hi:bacon}}
[![bacon~crates.io][c~bacon~crates.io~badge]][c~bacon~crates.io]
[![bacon~repo][c~bacon~repo~badge]][c~bacon~repo]
[![bacon~lib.rs][c~bacon~lib.rs~badge]][c~bacon~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`bacon`][c~bacon~docs]↗{{hi:bacon}} is a background rust code checker. It is designed for minimal interaction, so that we can just let it run, alongside your editor, and be notified of warnings, errors, or test failures in your Rust code.

```sh
# Install or Update `bacon`
cargo install --locked bacon

# Check the current project
bacon

# Run `clippy` instead of `cargo check`
bacon clippy
```

## Related Topics {#related-topics .skip}

Consider [`cargo-make`][c~cargo-make~docs]↗{{hi:cargo-make}} if we want something with a bulkier syntax but more cross-platform portability.

See [[building | Building]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
