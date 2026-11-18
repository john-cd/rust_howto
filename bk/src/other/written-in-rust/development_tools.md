<!-- TODO Reviewed ??-??-2026. -->

# Development Tools Written in Rust

{{#include development_tools.incl.md}}

## Version control tools {#jujutsu}

[Jujutsu (`jj`)][jj~repo] is a Git-compatible version control system written in Rust. [Steve Klabnik's Jujutsu tutorial][jujutsu-tutorial~website] and the [JJ workshop][jj-workshop~repo] provide guided introductions. [`gg`][gg~repo] provides a graphical interface for Jujutsu. [Gitoxide][gitoxide~repo] implements Git in Rust and exposes its functionality through libraries and command-line tools.

## Build tools {#build-tools}

[Pants][pantsbuild~website]↗ is a polyglot build system. Its execution engine is written in Rust, while build rules are written in typed Python. This split lets Pants use Rust for performance-critical work while keeping project-specific rules accessible; see the [Pants engine documentation][pants~engine~docs].

[`starlark-rust`][starlark-rust~repo]↗ implements the Starlark configuration language in Rust. Starlark is used by build systems including Bazel, Buck, and Buck2.

## JavaScript tooling {#javascript-tooling}

[Volta][volta~project~repo] is a JavaScript toolchain manager written in Rust. Its maintainers have ended active development.

[Deno][deno~website]↗ is a JavaScript, TypeScript, and WebAssembly runtime built on V8, Rust, and Tokio.

[Bun][bun~website]↗ combines a JavaScript runtime, package manager, bundler, and test runner. [Bun 1.4][bun~rust-release]↗ moved the runtime and tools from Zig to Rust and retained JavaScriptCore as the JavaScript engine.

[Biome][biome~repo]↗ provides a formatter and linter for web projects. [Oxc][oxc~repo]↗ provides JavaScript and TypeScript tooling, including a parser, linter, and transformer.

### SWC compiler {#swc}

{{#badges swc}}

[`swc`][c~swc~repo]↗ (Speedy Web Compiler) is a Rust compiler and toolkit for JavaScript and TypeScript. See also the [book's guide to language parsers and compilers][p~programming_languages].

## Code editors {#code-editors}

### Lapce {#lapce}

[Lapce][lapce~website]↗ is a cross-platform code editor written in Rust.

### Zed {#zed}

[Zed][c~zed~repo]↗ is a code editor with built-in collaboration.

### Helix {#helix}

[Helix][helix~repo]↗ is a modal text editor with built-in language-server support. [rust-analyzer][rust-analyzer~repo]↗ provides Rust diagnostics, completion, and navigation to editors through the Language Server Protocol.

## Terminal emulators {#terminal-emulators}

[`Alacritty`][alacritty~repo]↗ is a cross-platform, OpenGL terminal emulator.

[`WezTerm`][wezterm~repo]↗ is a GPU-accelerated, cross-platform terminal emulator and multiplexer.

[Warp][warp~repo]↗ is a terminal application with AI-assisted workflows.

## Command-line utilities {#utilities}

- [`ripgrep`][ripgrep~repo]↗ searches directory trees for regular expressions and respects Git ignore rules.
- [`fd`][fd~repo]↗ searches for files and directories by name.
- [`bat`][bat~repo]↗ displays files with syntax highlighting and Git change markers.
- [uutils coreutils][uutils-coreutils~repo]↗ reimplements GNU core utilities such as `cp` and `sort` in Rust.

- [`lsd`][c~lsd~repo]↗ is a modern `ls` alternative.
- [`fish`][fish-shell~repo]↗ is a command-line shell. The fish team's [“Fish 4.0: The Fish of Theseus”][blog~fishshell-rustport] article describes its component-by-component Rust port.
- [`nushell`][nushell~repo]↗ is a structured-data shell.
- [`zoxide`][c~zoxide~docs]↗ is a directory-jumping tool.
- [`dust`][c~dust~repo]↗ presents disk-usage information.
- [`bottom`][c~bottom~repo]↗ is a cross-platform system and process monitor.
- [`sd`][c~sd~repo]↗ is a find-and-replace command-line tool.
- [`starship`][c~starship~repo]↗ is a customizable prompt for multiple shells.
- [`broot`][c~broot~repo]↗ is an interactive directory navigator.
- [`navi`][c~navi~repo]↗ is an interactive command-line cheatsheet.
- [`grex`][c~grex~repo]↗ generates regular expressions from example strings.
- [`yazi`][yazi~repo]↗ is a terminal file manager.
- [`kget`][c~kget~docs]↗ is a download manager and library.

  [![kget][c~kget~docs~badge]][c~kget~docs] [![kget~crates.io][c~kget~crates.io~badge]][c~kget~crates.io] [![kget~repo][c~kget~repo~badge]][c~kget~repo] [![kget~lib.rs][c~kget~lib.rs~badge]][c~kget~lib.rs]

  [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command-line utilities}}

## Repository summaries and fuzzy finding {#terminal-utilities}

[`onefetch`][c~onefetch~repo]↗ is a command-line Git information tool that summarizes a repository in the terminal.

[`television`][c~television~docs]↗ is a general-purpose fuzzy finder TUI. It integrates with your shell and can search files, Git repositories, environment variables, container images, and other configured data sources.

## Package and project tools {#package-tools}

[`cargo-binstall`][c~cargo-binstall~repo]↗ installs prebuilt Rust binaries when a compatible artifact is available.

[`cargo-update`][c~cargo-update~crates.io]↗ provides commands for updating installed Cargo packages.

[`cargo-hakari`][c~cargo-hakari~crates.io]↗ helps unify dependency features across a Cargo workspace.

[`kondo`][kondo~repo]↗ finds build artifacts and dependency directories that can be cleaned from projects.

[Chef Habitat][habitat~repo]↗ packages, deploys, and supervises applications.

## Related Topics {#related-topics .skip}

- [Version control][p~version_control] covers Git tasks in the main book.
- [Shells and command-line utilities][book-cli~chapter] covers more command-line tools.
- [Python tools](python_tools.md) covers Python environments, linting, and packaging.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
