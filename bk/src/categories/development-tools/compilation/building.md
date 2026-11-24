<!-- TODO Reviewed ??-??-2026. -->

# Build Rust Code {#build-code}

{{#include building.incl.md}}

| Concern | Where to start |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Build a package | Use `cargo check` for fast feedback, `cargo build` to compile, and `cargo run` to build and run a binary. |
| Build scripts | Add `build.rs` only when a build needs to generate files, inspect the environment, or compile native code. See the [Cargo build-script reference][book~cargo~build-script]↗. |
| Build profiles | Set development and release profile options in `Cargo.toml`; measure changes against the workload you care about. |
| Compiler and linker configuration | Prefer Cargo's target and profile configuration. Keep machine-specific paths out of shared configuration. |
| Repeated project tasks | Use a small `justfile` or a dedicated task runner when ordinary Cargo commands are not enough; see the [Cargo plugin build guide][p~building]. |

For cross-compilation, linker choices, and compile-time optimization, use the focused [cross-compilation][p~cross-compilation] and [compilation performance][p~performance] guides. This chapter covers Cargo's normal build flow; the separate [Cargo plugin build guide][p~building] covers task runners such as `cargo-make`, `cargo-xtask`, and `xshell`.

## Save and run project-specific Commands with the `just` Command Runner {#just}

{{#badges just development-tools="Development tools" command-line-utilities="Command line utilities"}}{{hi:Command-line}}{{hi:Development}}{{hi:Runner}}{{hi:Task}}{{hi:Utility}}

[`just`][c~just~docs]↗{{hi:just}}{{hi:just}} is a command runner{{hi:Command runner}}. It is a Rust-based equivalent to [`make`][make~website]↗{{hi:make}} without the ability to detect file changes, but with significantly fewer syntactic warts.

Consult the [Just programmer's manual][c~just~programmer-manual]↗.

### Create a `justfile` {#create-a-justfile .skip}

Place the following example `justfile`{{hi:justfile}} in the root folder of your project. Run [`just`][c~just~website]↗{{hi:just}} to see a list of recipes. Run `just <recipe>` to execute the corresponding recipe.

```makefile
{{#include examples/justfile}}
```

Run `just build` to check and build the project, or pass a name filter with `just test parses_input`. On Windows, run these recipes from Git Bash with its tools on PATH. From a child directory, run `just where` to see the difference between the invocation directory and the directory where recipes run.

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

{{#badges bacon development-tools="Development tools" command-line-utilities="Command line utilities"}}

[`bacon`][c~bacon~docs]↗{{hi:bacon}} checks Rust code in the background. Leave it running alongside your editor to see warnings, errors, and test failures as you work.

```sh
# Install or Update `bacon`
cargo install --locked bacon

# Check the current project
bacon

# Run `clippy` instead of `cargo check`
bacon clippy
```

## Related Topics {#related-topics .skip}

Consider [`cargo-make`][c~cargo-make~docs]↗{{hi:cargo-make}} for tasks that need to run across platforms.

See also the [Rust Compilation section](../index.md#rust-compilation) in the Development Tools overview.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
