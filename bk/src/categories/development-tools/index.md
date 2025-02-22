# Tools

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

Tools that provide developer-facing features such as testing, debugging, linting, performance profiling, autocompletion, formatting, and more.

[Rust tools][rust-tools]{{hi:Rust tools}}⮳

## Cargo

{{#include cargo/cargo.incl.md}}

{{#include cargo/crate_registries.incl.md}}

{{#include cargo/package_layout.incl.md}}

## Documentation

{{#include documentation/documentation.incl.md}}

{{#include documentation/mdbook.incl.md}}

{{#include documentation/badges.incl.md}}

## Formatting

{{#include formatting/formatting.incl.md}}

## Installation

{{#include installation/rustup.incl.md}}

{{#include installation/install.incl.md}}

## Other

{{#include other/code_build.incl.md}}

{{#include other/code_verification.incl.md}}

{{#include other/miri.incl.md}}

{{#include other/other.incl.md}}

## Versioning

{{#include versioning/versioning.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools/index: add (P2)](https://github.com/john-cd/rust_howto/issues/301)
[index: reorganize; dedupe alternatives / log / config_log (P1)](https://github.com/john-cd/rust_howto/issues/319)

Build System/Package Manager: [`cargo`][c-cargo]⮳{{hi:cargo}}
Formatter: [`rustfmt`][c-rustfmt]⮳{{hi:rustfmt}}
Linter: [`clippy`][c-clippy]⮳{{hi:clippy}}
Testing Framework: `std::test` (built-in), [`rstest`][c-rstest]⮳{{hi:rstest}} (for data-driven tests)
Debugging: `gdb`, `lldb` (often used via IDE integration), `cargo-debug`
Profiling: `cargo flamegraph`, `perf` (system profiler)
Documentation Generator: `cargo doc`
Code Coverage: [`grcov`][c-grcov]⮳{{hi:grcov}}, `tarpaulin`
Continuous Integration: often uses CI platforms directly; no specific Rust crates needed, but [`xtask`][c-xtask]⮳{{hi:xtask}} can help manage CI tasks
Code Analysis: `cargo clippy`, `rust-analyzer` (for Language Server Protocol support in IDEs)

[`fd-find`][c-fd_find]⮳{{hi:fd-find}} as a more human-friendly alternative to `find` which, by default, ignores paths listed in things like .gitignore and mimics Vim's smartcase option.

[`hyperfine`][c-hyperfine]⮳{{hi:hyperfine}} as an analogue to the UNIX time command which can do warm-up runs, run the command multiple times to do statistical outlier detection, display a progress bar for the multiple runs, export results to CSV/JSON/etc., parameterize the runs, etc.

[`miniserve`][c-miniserve]⮳{{hi:miniserve}} as a simple, easy way to serve up some files or accept some uploads over HTTP.

[`ripgrep`][c-ripgrep]⮳{{hi:ripgrep}} for fast searching of file contents

[`rust-script`][c-rust_script]⮳{{hi:rust-script}} as a way to quickly write little single-file programs in Rust without having to spin up a whole project.

[`skim`][c-skim]⮳{{hi:skim}} as a Rust clone of fzf with some additional features. (Including being usable as a library you can embed in your own programs)

[`tokei`][c-tokei]⮳{{hi:tokei}} for gathering statistics about a codebase (i.e. number of files, lines, lines of code, lines of comments, and lines of blanks, per language)

[`xd`][c-xd]⮳{{hi:xd}} as an alternative to xxd that doesn't have as many features, but renders un-printable characters in a reversible "codepage 437 plus a symbol for NULL" mapping to ensure that all patterns in the visualization of binary files are visible... not just ones that occur in printable characters.

[`cross`][c-cross]⮳{{hi:cross}}
</div>
