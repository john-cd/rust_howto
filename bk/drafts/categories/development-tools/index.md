# Development Tools

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Tools that provide developer-facing features such as testing, debugging, linting, performance profiling, autocompletion, formatting, and more.

| Topic | Rust Crates |
|---|---|
| Build System/Package Manager | [`cargo`][c~cargo~docs]↗{{hi:cargo}} |
| Formatter | [`rustfmt`][c~rustfmt~docs]↗{{hi:rustfmt}} |
| Linter | [`clippy`][c~clippy~docs]↗{{hi:clippy}} |
| Testing Framework | [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} (built-in), [`rstest`][c~rstest~docs]↗{{hi:rstest}} (for data-driven tests) |
| Debugging | [`gdb`][c~gdb~docs]↗{{hi:gdb}}, [`lldb`][c~lldb~docs]↗{{hi:lldb}} (often used via IDE integration), `cargo-debug` |
| Profiling | `cargo flamegraph`, [`perf`][c~perf~docs]↗{{hi:perf}} (system profiler) |
| Documentation Generator | [`cargo doc`][book~cargo~cargo-doc]↗{{hi:cargo doc}} |
| Code Coverage | [`grcov`][c~grcov~docs]↗{{hi:grcov}}, [`tarpaulin`][c~cargo-tarpaulin~docs]↗{{hi:tarpaulin}} |
| Continuous Integration | often uses CI platforms directly; no specific Rust crates needed, but [`xtask`][c~xtask~docs]↗{{hi:xtask}} can help manage CI tasks |
| Code Analysis | `cargo clippy`, `rust-analyzer` (for Language Server Protocol support in IDEs) |

See also [Rust tools][rust-tools]{{hi:Rust tools}}↗.

## Cargo

{{#include cargo/cargo.incl.md}}

{{#include cargo/crate_registries.incl.md}}

{{#include cargo/package_layout.incl.md}}

## Rust Compilation

{{#include compilation/reduce_compilation_duration.incl.md}}

{{#include compilation/faster_linking.incl.md}}

## Documentation

{{#include documentation/documentation.incl.md}}

{{#include documentation/mdbook.incl.md}}

{{#include documentation/badges.incl.md}}

## Formatting

{{#include formatting/formatting.incl.md}}

## Installation

{{#include installation/rustup.incl.md}}

{{#include installation/install.incl.md}}

## Versioning

{{#include versioning/versioning.incl.md}}

## Rust Code Cross-compilation

{{#include cross-compilation/cross_compilation.incl.md}}

See also [[cross_compiling | Cross Compiling (Cargo Plugins)]].

## Transpilation into Rust

{{#include transcompilation/transpilers.incl.md}}

## Other

{{#include other/code_build.incl.md}}

{{#include other/code_verification.incl.md}}

{{#include other/miri.incl.md}}

{{#include other/other.incl.md}}

## Useful Utilities

| Utilities |
|---|
| [`fd-find`][c~fd-find~docs]↗{{hi:fd-find}} as a more human-friendly alternative to [`find`][c~find~docs]↗{{hi:find}} which, by default, ignores paths listed in things like [`.gitignore`][git-gitignore~website]↗{{hi:.gitignore}} and mimics Vim's smartcase option. |
| [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} as an analogue to the UNIX time command which can do warm-up runs, run the command multiple times to do statistical outlier detection, display a progress bar for the multiple runs, export results to CSV/JSON/etc., parameterize the runs, etc. |
| [`miniserve`][c~miniserve~docs]↗{{hi:miniserve}} as a simple, easy way to serve up some files or accept some uploads over HTTP. |
| [`ripgrep`][c~ripgrep~docs]↗{{hi:ripgrep}} is excellent for fast searching of file contents. |
| [`rust-script`][c~rust-script~docs]↗{{hi:rust-script}} as a way to quickly write little single-file programs in Rust without having to spin up a whole project. |
| [`skim`][c~skim~docs]↗{{hi:skim}} is a Rust clone of [`fzf`][fzf~github]↗{{hi:fzf}} with some additional features. |
| [`tokei`][c~tokei~docs]↗{{hi:tokei}} gathers statistics about a codebase (i.e. number of files, lines, lines of code, lines of comments, and lines of blanks, per language). |
| [`xd`][c~xd~docs]↗{{hi:xd}} as an alternative to 'xxd' that doesn't have as many features, but renders un-printable characters in a reversible "codepage 437 plus a symbol for NULL" mapping to ensure that all patterns in the visualization of binary files are visible... not just ones that occur in printable characters. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[index: reorganize; dedupe alternatives / [`log`][c~log~docs]↗{{hi:log}} / config_log NOW](https://github.com/john-cd/rust_howto/issues/319)

## Editors

- [`lapce`][lapce~github]↗: Lightning-fast and Powerful Code Editor written in Rust.
- [`intellij-rust`][intellij-rust~github]↗: Rust plugin for the IntelliJ Platform.

## Registries {#skip}

- [`alexandrie`][c~alexandrie~github]↗: An alternative crate registry, implemented in Rust.
- [`meuse`][meuse~github]↗: A private Cargo crate registry, for Rust.
- [`kellnr`][kellnr~github]↗: The registry for Rust crates.
- [`panamax`][c~panamax~github]↗: Mirror rustup and crates.io repositories, for offline Rust and cargo usage..

## Tests

- [`assertables`](https://docs.rs/assertables/latest/assertables)↗.
- [`proptest`][c~proptest~github]↗: Hypothesis-like property testing for Rust.

## Cargo Plugins

- [`cargo-semver-checks`][c~cargo-semver-checks~crates.io]↗.
- [`cargo-c`][c~cargo-c~crates.io]↗.
- [`kondo`][kondo~github]↗: Cleans dependencies and build artifacts from your projects.
- [`cargo-sweep`][c~cargo-sweep~github]↗: A cargo subcommand for cleaning up unused build files generated by Cargo.
- [`cargo-update`][c~cargo-update~github]↗: A cargo subcommand for checking and applying updates to installed executables.
- [`cargo-xtask`][c~cargo-xtask~github]↗.
- [`cargo-script`][c~cargo-script~crates.io]↗.
- [`cargo-play`][c~cargo-play~crates.io]↗.
- [`sccache-action`][sccache-action~github]↗: `sccache` github action.
- [`cargo-spellcheck`](https://github.com/drahnr/cargo-spellcheck)↗: Checks all your documentation for spelling and grammar mistakes with hunspell and a nlprule based checker for grammar.
- [`cargo-limit`: Productivity improvements for Rust ecosystem: warnings are skipped until errors are fixed, LSP-independent Neovim integration, etc.][cargo-limit~github]↗.
- [`cargo-bloat`][c~cargo-bloat~github]↗: Find out what takes most of the space in your executable.
- [`cargo-generate`][c~cargo-generate~crates.io]↗.
- [`cargo-workspaces`][c~cargo-workspaces~crates.io]↗.

## Linkers

- [Mold][mold~github]↗: A Modern Linker.

## Cross Compilation

- [Cross-Platform Development][rustmeup-tutorials-cross-platform-development~website]↗.
- [Cross-compilation in Rust][blog~kerkour-rust-cross-compilation]↗.

## Live Reload

- [So you want to live-reload Rust][blog~fasterthanli.me-so-you-want-to-live-reload-rust]↗.
- [`watchexec`][c~watchexec~github]↗: Executes commands in response to file modifications.

## CD / CI

- [`act`][act~github]↗: Run your GitHub Actions locally.

## Others

- [`vscode-just`][vscode-just~github]↗: VSCode syntax highlighting for just files.

- [`dotbot`][dotbot~github]↗: A tool that bootstraps your dotfiles.

- [Bazel at Enterprise Scale | BuildBuddy][buildbuddy~website]↗.

- [The Pants Build System][pants~github]↗.

- [`mini-docker-rust`][mini-docker-rust~github]↗: Very small rust docker image.

- [`contracts`][c~contracts~crates.io]↗.

- [Vagrant by HashiCorp][vagrantup~website]↗.

- [`include_dir` - Rust dev tool][c~include_dir~lib.rs]↗: An evolution of the `include_str!()` and `include_bytes!()` macros for embedding an entire directory tree into your binary.

</div>
