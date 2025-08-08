# Development Tools

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Tools that provide developer-facing features such as testing, debugging, linting, performance profiling, autocompletion, formatting, and more.

| Topic | Rust Crates |
|---|---|
| Build System/Package Manager | [`cargo`][c~cargo~docs]↗{{hi:cargo}} |
| Formatter | [`rustfmt`][c~rustfmt~docs]↗{{hi:rustfmt}} |
| Linter | [`clippy`][c~clippy~docs]↗{{hi:clippy}} |
| Testing Framework | [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} (built-in), [`rstest`][c~rstest~docs]↗{{hi:rstest}} (for data-driven tests) |
| Debugging | [`gdb`][c~gdb~docs]↗{{hi:gdb}}, [`lldb`][c~lldb~docs]↗{{hi:lldb}} (often used via IDE integration), `cargo-debug` |
| Profiling | `cargo flamegraph`, [`perf`][c~perf~docs]↗{{hi:perf}} (system profiler) |
| Documentation Generator | [`cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)↗{{hi:cargo doc}} |
| Code Coverage | [`grcov`][c~grcov~docs]↗{{hi:grcov}}, [`tarpaulin`][c~tarpaulin~docs]↗{{hi:tarpaulin}} |
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
| [`fd-find`][c~fd-find~docs]↗{{hi:fd-find}} as a more human-friendly alternative to [`find`][c~find~docs]↗{{hi:find}} which, by default, ignores paths listed in things like [`.gitignore`](https://git-scm.com/docs/gitignore)↗{{hi:.gitignore}} and mimics Vim's smartcase option. |
| [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} as an analogue to the UNIX time command which can do warm-up runs, run the command multiple times to do statistical outlier detection, display a progress bar for the multiple runs, export results to CSV/JSON/etc., parameterize the runs, etc. |
| [`miniserve`][c~miniserve~docs]↗{{hi:miniserve}} as a simple, easy way to serve up some files or accept some uploads over HTTP. |
| [`ripgrep`][c~ripgrep~docs]↗{{hi:ripgrep}} is excellent for fast searching of file contents. |
| [`rust-script`][c~rust-script~docs]↗{{hi:rust-script}} as a way to quickly write little single-file programs in Rust without having to spin up a whole project. |
| [`skim`][c~skim~docs]↗{{hi:skim}} is a Rust clone of [`fzf`](https://github.com/junegunn/fzf)↗{{hi:fzf}} with some additional features. |
| [`tokei`][c~tokei~docs]↗{{hi:tokei}} gathers statistics about a codebase (i.e. number of files, lines, lines of code, lines of comments, and lines of blanks, per language). |
| [`xd`][c~xd~docs]↗{{hi:xd}} as an alternative to 'xxd' that doesn't have as many features, but renders un-printable characters in a reversible "codepage 437 plus a symbol for NULL" mapping to ensure that all patterns in the visualization of binary files are visible... not just ones that occur in printable characters. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[index: reorganize; dedupe alternatives / [`log`][c~log~docs]↗{{hi:log}} / config_log NOW](https://github.com/john-cd/rust_howto/issues/319)

## Editors

- [lapce: Lightning-fast and Powerful Code Editor written in Rust](https://github.com/lapce/lapce)
- [intellij-rust: Rust plugin for the IntelliJ Platform](https://github.com/intellij-rust/intellij-rust)

## Registries {#skip}

- [alexandrie: An alternative crate registry, implemented in Rust.](https://github.com/Hirevo/alexandrie)
- [meuse: A private Cargo crate registry, for Rust](https://github.com/mcorbin/meuse)
- [kellnr: The registry for Rust crates](https://github.com/kellnr/kellnr)
- [panamax: Mirror rustup and crates.io repositories, for offline Rust and cargo usage.](https://github.com/panamax-rs/panamax)

## Tests

- [assertables](https://docs.rs/assertables/latest/assertables/)
- [proptest: Hypothesis-like property testing for Rust](https://github.com/proptest-rs/proptest)

## Cargo Plugins

- [`cargo-semver-checks`](https://crates.io/crates/cargo-semver-checks)
- [`cargo-c`](https://crates.io/crates/cargo-c)
- [`kondo`: Cleans dependencies and build artifacts from your projects.](https://github.com/tbillington/kondo)
- [`cargo-sweep`: A cargo subcommand for cleaning up unused build files generated by Cargo](https://github.com/holmgr/cargo-sweep)
- [`cargo-update`: A cargo subcommand for checking and applying updates to installed executables](https://github.com/nabijaczleweli/cargo-update)
- [`cargo-xtask`](https://github.com/matklad/cargo-xtask)
- [`cargo-script`](https://crates.io/crates/cargo-script)
- [`cargo-play`](https://crates.io/crates/cargo-play)
- [`sccache-action`: sccache github action](https://github.com/Mozilla-Actions/sccache-action)
- [`cargo-spellcheck`: Checks all your documentation for spelling and grammar mistakes with hunspell and a nlprule based checker for grammar](https://github.com/drahnr/cargo-spellcheck)
- [`cargo-limit`: Productivity improvements for Rust ecosystem: warnings are skipped until errors are fixed, LSP-independent Neovim integration, etc.](https://github.com/cargo-limit/cargo-limit)
- [`cargo-bloat`: Find out what takes most of the space in your executable.](https://github.com/RazrFalcon/cargo-bloat)
- [`cargo-generate`](https://crates.io/crates/cargo-generate)
- [`cargo-workspaces`](https://crates.io/crates/cargo-workspaces)

## Linkers

- [Mold: A Modern Linker](https://github.com/rui314/mold)

## Cross Compilation

- [Cross-Platform Development](https://rustmeup.com/tutorials/cross-platform-development)
- [Cross-compilation in Rust](https://kerkour.com/rust-cross-compilation)

## Live Reload

- [So you want to live-reload Rust](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)
- [`watchexec`: Executes commands in response to file modifications](https://github.com/watchexec/watchexec)

## CD / CI

- [`act`: Run your GitHub Actions locally](https://github.com/nektos/act)

## Others

- [`vscode-just`: VSCode syntax highlighting for just files](https://github.com/nefrob/vscode-just)

- [`dotbot`: A tool that bootstraps your dotfiles](https://github.com/anishathalye/dotbot)

- [Bazel at Enterprise Scale | BuildBuddy](https://www.buildbuddy.io/)

- [The Pants Build System](https://github.com/pantsbuild/pants)

- [`mini-docker-rust`: Very small rust docker image](https://github.com/kpcyrd/mini-docker-rust)

- [`contracts`](https://crates.io/crates/contracts)

- [Vagrant by HashiCorp](https://www.vagrantup.com/)

- [`include_dir` - Rust dev tool](https://lib.rs/crates/include_dir) An evolution of the include_str!() and include_bytes!() macros for embedding an entire directory tree into your binary.

</div>
