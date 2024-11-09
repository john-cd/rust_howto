# Tools

Tools that provide developer-facing features such as testing, debugging, linting, performance profiling, autocompletion, formatting, and more.

[Rust tools][rust-tools]{{hi:Rust tools}}⮳

## Cargo

{{#include cargo/index.incl.md}}

## Documentation

{{#include documentation/index.incl.md}}

## Formatting

{{#include formatting/index.incl.md}}

## Installation

{{#include installation/index.incl.md}}

## Other

{{#include other/index.incl.md}}

## Versioning

{{#include versioning/index.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO add

cargo-afl for fuzzing

cargo-asm and cargo-expand to investigate what the compiler generates from your code (cargo-expand shows the expanded output from macros)

cargo-audit for checking whether any of your dependencies are of a version that has a security advisory out against them.

cargo-bloat for identifying what's contributing to your binary's size (eg. modules with generic functions or macros not designed with size-efficiency in mind)

cargo-cache, cargo-sweep, and cargo-prune for keeping disk consumption by build artifacts and other regeneratable files under control.

cargo-deadlinks Check cargo doc output for broken old-style/manual intra-doc links.

cargo-edit for cargo add <dependency>, cargo rm <dependency> and cargo upgrade to update your Cargo.toml's versions. (Functionality eventually planned to be part of Cargo itself)

cargo-geiger for identifying dependencies with unsafe code so you can either audit them or find alternatives if you don't feel skilled enough to do your own auditing.

cargo-modules for rendering a tree or Graphviz graph of the modules within a crate

cargo-outdated for listing packages that have newer versions than what your Cargo.toml and Cargo.lock are pinning.

cargo-tree to investigate dependencies (I like to use -d to list crates where more than one version is getting pulled in and what's pulling each version in.)

cargo-update to provide a cargo install-update to check for and install new versions of cargo install'd commands.

cargo-watch to re-run a command every time the source changes. (eg. cargo test)

fd-find as a more human-friendly alternative to find which, by default, ignores paths listed in things like .gitignore and mimics Vim's smartcase option.

flamegraph as an easy way to generate flamegraphs for visualizing performance profiles of Rust programs.

hyperfine as an analogue to the UNIX time command which can do warm-up runs, run the command multiple times to do statistical outlier detection, display a progress bar for the multiple runs, export results to CSV/JSON/etc., parameterize the runs, etc.

just as a Rust-based equivalent to make without the "have files changed" detection but with significantly fewer syntactic warts. (See cargo-make if you want something with a bulkier syntax but more cross-platform portability)

miniserve as a simple, easy way to serve up some files or accept some uploads over HTTP.

ripgrep for fast searching of file contents

rust-script as a way to quickly write little single-file programs in Rust without having to spin up a whole project.

skim as a Rust clone of fzf with some additional features. (Including being usable as a library you can embed in your own programs)

tokei for gathering statistics about a codebase (i.e. number of files, lines, lines of code, lines of comments, and lines of blanks, per language)

xd as an alternative to xxd that doesn't have as many features, but renders un-printable characters in a reversible "codepage 437 plus a symbol for NULL" mapping to ensure that all patterns in the visualization of binary files are visible... not just ones that occur in printable characters.)

cargo-about, cargo-deny, cargo-license, or cargo-lichking for license compliance management

cargo-audit and cargo-sweep

cargo-spellcheck

cross is excellent by all accounts.
</div>
