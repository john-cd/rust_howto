<!-- TODO Reviewed ??-??-2026 -->

# Development Tools Written in Rust

{{#include development_tools.incl.md}}

This page maps developer-facing tools written wholly or partly in Rust, along with companion projects that help use them. Some projects use Rust for performance-critical components and another language for plugins; check each project's documentation for its architecture and current maintenance status.

## Version control tools {#jujutsu}

[Jujutsu (`jj`)][jj~repo] is a Git-compatible version control system written in Rust. [Steve Klabnik's Jujutsu tutorial][jujutsu-tutorial~website] and the [JJ workshop][jj-workshop~repo] provide guided introductions. [`gg`][gg~repo] is a graphical companion for JJ.

## Build tools {#build-tools}

[Pants][pantsbuild~website]↗ is a polyglot build system. Its execution engine is written in Rust, while build rules are written in typed Python. This split lets Pants use Rust for performance-critical work while keeping project-specific rules accessible; see the [Pants engine documentation][pants~engine~docs].

[`starlark-rust`][starlark-rust~repo]↗ implements the Starlark configuration language in Rust. Starlark is used by build systems including Bazel, Buck, and Buck2.

## JavaScript tooling {#javascript-tooling}

[Volta][volta~project~repo] is a Rust-based JavaScript toolchain manager. Its README now marks the project unmaintained and recommends migration, so check its current status before adopting it.

[Deno][deno~website]↗ is a JavaScript, TypeScript, and WebAssembly runtime built on V8, Rust, and Tokio.

[Bun][bun.sh~website]↗ uses JavaScriptCore; its current runtime documentation describes the runtime and transpiler as written in Rust. See the [Bun runtime documentation][bun~runtime~docs] for the current architecture.

### SWC compiler {#swc}

{{#badges swc}}

[`swc`][c~swc~repo]↗ (Speedy Web Compiler) is a Rust compiler and toolkit for JavaScript and TypeScript. See also [[programming_languages | Programming Languages]] and its parser tools.

## Code editors {#code-editors}

### Lapce {#lapce}

[![lapce~website][lapce~website~badge]][lapce~website]{{hi:Lapce}}

[Lapce][lapce~website]↗ is a cross-platform code editor written in Rust.

### Zed {#zed}

[![zed~crates.io][c~zed~crates.io~badge]][c~zed~crates.io]
[![zed~repo][c~zed~repo~badge]][c~zed~repo]
[![zed~lib.rs][c~zed~lib.rs~badge]][c~zed~lib.rs]

[`Zed`][c~zed~repo]↗ is a high-performance, collaborative code editor from the creators of Atom and Tree-sitter.

### Helix {#helix}

[`Helix`][helix~repo]↗ is a modal text editor with built-in language-server support.

## Terminal emulators {#terminal-emulators}

[`Alacritty`][alacritty~repo]↗ is a cross-platform, OpenGL terminal emulator.

[`WezTerm`][wezterm~repo]↗ is a GPU-accelerated, cross-platform terminal emulator and multiplexer.

[`Warp`][warp~repo]↗ is a terminal application whose Rust codebase is linked from its project repository.

## Command-line utilities {#utilities}

These command-line tools cover shells, file navigation, search, monitoring, prompts, and downloads:

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

## Terminal utilities

[`onefetch`][c~onefetch~repo]↗ is a command-line Git information tool that summarizes a repository in the terminal.

[`television`][c~television~docs]↗ is a general-purpose fuzzy finder TUI. It integrates with your shell and can search files, Git repositories, [environment variables][p~environment-variables], container images, and other configured data sources.

## Package and project tools {#package-tools}

[`cargo-binstall`][c~cargo-binstall~repo]↗ installs prebuilt Rust binaries when a compatible artifact is available.

[`cargo-update`][c~cargo-update~crates.io]↗ provides commands for updating installed Cargo packages.

[`cargo-hakari`][c~cargo-hakari~crates.io]↗ helps unify dependency features across a Cargo workspace.

[`kondo`][kondo~repo]↗ finds build artifacts and dependency directories that can be cleaned from projects.

[`Habitat`][habitat~repo]↗ is a project for building, deploying, and managing applications.

## Related Topics {#related-topics .skip}

- [[programming_languages | Programming Languages]].
- [[command-line-interface | Command Line Interface]].
- [[version_control | Version Control]].
- [[cross-platform | Cross-platform Development]].
- [[gui | GUI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries

- The [Awesome Rust Tools][awesome-rust-tools~repo]↗ and [Awesome Alternatives in Rust][awesome-alternatives-in-rust~repo]↗ lists are broader catalogs, not endorsements or maintenance guarantees.

</div>
