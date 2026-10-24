<!-- TODO Reviewed ??-??-2026 -->

# Rustup

{{#include rustup.incl.md}}

## Install and Manage Rust Toolchains with `rustup` {#rustup}

[![rustup][rustup~website~badge]][rustup~website] [Rustup documentation][rustup~documentation]↗ [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`rustup`][rustup~website]↗{{hi:rustup}} is a toolchain multiplexer. It installs, manages, and upgrades versions of the rust compiler [`rustc`][book~rustc]↗{{hi:rustc}}, the Rust package manager [`cargo`][c~cargo~docs]↗{{hi:cargo}}, the Rust linter [`clippy`][c~clippy~docs]↗{{hi:clippy}}, the Rust code formatter [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}}, etc.

More precisely, [`rustup`][rustup~website]↗{{hi:rustup}} can install and manage multiple Rust toolchains and presents them all through a single set of tools installed to `~/.cargo/bin`. The [`rustc`][book~rustc]↗{{hi:rustc}} and [`cargo`][c~cargo~docs]↗{{hi:cargo}} executables installed e.g. in `~/.cargo/bin` are proxies that delegate to the real toolchain.

[`rustup`][rustup~website]↗{{hi:rustup}} is similar in purpose to Python's [`pyenv`][pyenv~repo]↗{{hi:pyenv}} or Node's [`nvm`][nvm~repo]↗{{hi:nvm}}. Use these commands from a terminal:

| Task                                  | Command                                       | Notes                                                                                                               |
| ------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Check installed and active toolchains | `rustup show`                                 | The active toolchain is selected for the current directory.                                                         |
| List installed toolchains             | `rustup toolchain list`                       | Shows toolchains installed through rustup.                                                                          |
| Install or update toolchains          | `rustup update stable`                        | Omit `stable` to update every installed toolchain.                                                                  |
| Choose the default toolchain          | `rustup default stable`                       | Use `rustup default --help` for accepted toolchain names.                                                           |
| Add a component                       | `rustup component add clippy`                 | Components such as `rustfmt` are distributed per toolchain; availability varies.                                    |
| List installed targets                | `rustup target list --installed`              | A target standard library alone may not include the linker or system libraries needed to produce a runnable binary. |
| Add a compilation target              | `rustup target add aarch64-unknown-linux-gnu` | Install a target-specific linker and other build tools when required.                                               |
| Open local Rust documentation         | `rustup doc`                                  | Add the `rust-docs` component if the documentation is not installed.                                                |

Toolchain selection can be scoped to a command with `cargo +stable test` or to a project with a `rust-toolchain.toml` file. A `rustup override` can also select a toolchain for a directory. The precedence is: command-line `+toolchain`, `RUSTUP_TOOLCHAIN`, the nearest project file or directory override, then the default toolchain. Confirm the result with `rustup show`.

## Related Topics {#related-topics .skip}

- [Install Rust and Cargo][rustup~documentation]↗.
- [Cargo][p~cargo] and [building Rust code][p~code-building].
- [Cross-compilation][p~cross-platform].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
</div>
