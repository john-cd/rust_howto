# Rustup

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[`rustup`][rustup-website]{{hi:rustup}}⮳ is a toolchain multiplexer. It installs and manages many Rust toolchains and presents them all through a single set of tools installed to `~/.cargo/bin`. The [`rustc`][rustc]{{hi:rustc}}⮳ and [`cargo`][c-cargo]{{hi:cargo}}⮳ executables installed e.g. in `~/.cargo/bin` are proxies that delegate to the real toolchain.

This is similar to Python's [`pyenv`][pyenv-github]{{hi:pyenv}}⮳ or Node's [`nvm`][nvm-github]{{hi:nvm}}⮳.

## Key commands

```sh
rustup help

# Show the help page for a subcommand (like toolchain)
rustup toolchain help

# Open the local documentation in your browser
rustup doc

# Update to a new version of Rust
rustup update

# Show which toolchain will be used in the current directory
rustup show

# Show which toolchain will be used in the current directory
rustup target list

# Overview of what is installed on your system
rustup toolchain list

# See a list of available and installed components.
rustup component list
```

[Rustup command examples][rustup-command-examples]⮳

## See also

[Rustup documentation][rustup-documentation]⮳

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
</div>
