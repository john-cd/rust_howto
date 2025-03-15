# Rustup

{{#include rustup.incl.md}}

## Install and Manage Rust Toolchains with `rustup` {#rustup}

[![rustup][rustup-website-badge]][rustup-website] [Rustup documentation][rustup-documentation]⮳ [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[`rustup`][rustup-website]{{hi:rustup}}⮳ is a toolchain multiplexer. It installs, manages, and upgrades versions of the rust compiler [`rustc`][rustc]⮳{{hi:rustc}}, the Rust package manager [`cargo`][c-cargo]⮳{{hi:cargo}}, the Rust linter [`clippy`][c-clippy]⮳{{hi:clippy}}, the Rust code formatter [`rustfmt`][c-rustfmt]⮳{{hi:rustfmt}}, etc.

More precisely, [`rustup`][c-rustup]⮳{{hi:rustup}} can install and manage multiple Rust toolchains and presents them all through a single set of tools installed to `~/.cargo/bin`. The [`rustc`][rustc]{{hi:rustc}}⮳ and [`cargo`][c-cargo]{{hi:cargo}}⮳ executables installed e.g. in `~/.cargo/bin` are proxies that delegate to the real toolchain.

[`rustup`][c-rustup]⮳{{hi:rustup}} is similar to Python's [`pyenv`][pyenv-github]{{hi:pyenv}}⮳ or Node's [`nvm`][nvm-github]{{hi:nvm}}⮳.

Key [`rustup` commands][rustup-command-examples]⮳ include the following:

```sh
# Rustup's help
rustup help

# Show the help page for a Subcommand (like `toolchain`)
rustup toolchain help

# Show which toolchain will be used in the current directory
rustup show

# Update to a new Version of Rust
rustup update

# Show which toolchain will be used in the current directory
rustup target list

# Overview of what is installed on your system
rustup toolchain list

# See a list of available and installed components.
rustup component list
```

Rustup also offers convenience commands:

```sh
# Open the local documentation in your browser
rustup doc
# May require `rustup component add rust-docs`
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[rustup: expand / clean up](https://github.com/john-cd/rust_howto/issues/302)
</div>
