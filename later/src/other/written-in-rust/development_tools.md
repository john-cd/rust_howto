<!-- TODO Reviewed ??-??-2026 -->

# Development Tools Written in Rust

{{#include development_tools.incl.md}}

This page maps developer-facing tools written wholly or partly in Rust, along with companion projects that help use them. Some projects use Rust for performance-critical components and another language for plugins; check each project's documentation for its architecture and current maintenance status.

## Version control tools {#jujutsu}

[Jujutsu (`jj`)][jj~repo] is a Git-compatible version control system written in Rust. [Steve Klabnik's Jujutsu tutorial][jujutsu-tutorial~website] and the [JJ workshop][jj-workshop~repo] provide guided introductions. [`gg`][gg~repo] is a graphical companion for JJ.

## Build tools {#build-tools}

[Pants][pantsbuild~website]↗ is a polyglot build system. Its execution engine is written in Rust, while build rules are written in typed Python. This split lets Pants use Rust for performance-critical work while keeping project-specific rules accessible; see the [Pants engine documentation][pants~engine~docs].

## JavaScript tooling {#javascript-tooling}

[Volta][volta~project~repo] is a Rust-based JavaScript toolchain manager. Its README now marks the project unmaintained and recommends migration, so check its current status before adopting it.

[Deno][deno~website]↗ is a JavaScript, TypeScript, and WebAssembly runtime built on V8, Rust, and Tokio.

[Bun][bun.sh~website]↗ uses JavaScriptCore; its current runtime documentation describes the runtime and transpiler as written in Rust. See the [Bun runtime documentation][bun~runtime~docs] for the current architecture.

### SWC compiler {#swc}

[![swc][c~swc~docs~badge]][c~swc~docs]{{hi:swc}}
[![swc~crates.io][c~swc~crates.io~badge]][c~swc~crates.io]
[![swc~repo][c~swc~repo~badge]][c~swc~repo]
[![swc~lib.rs][c~swc~lib.rs~badge]][c~swc~lib.rs]

[`swc`][c~swc~repo]↗ (Speedy Web Compiler) is a Rust compiler and toolkit for JavaScript and TypeScript. See also [[programming_languages | Programming Languages]] and its parser tools.

## Code editors {#code-editors}

### Lapce {#lapce}

[Lapce][lapce~website]↗ is a cross-platform code editor written in Rust.

### Zed {#zed}

[`Zed`][c~zed~repo]↗ is a high-performance, collaborative code editor from the creators of Atom and Tree-sitter.

## Terminal utilities {#utilities}

[`onefetch`][c~onefetch~repo]↗ is a command-line Git information tool that summarizes a repository in the terminal.

[`television`][c~television~docs]↗ is a general-purpose fuzzy finder TUI. It integrates with your shell and can search files, Git repositories, [environment variables][p~environment-variables], container images, and other configured data sources.

## Related Topics {#related-topics .skip}

- [[programming_languages | Programming Languages]].
- [[command-line-interface | Command Line Interface]].
- [[version_control | Version Control]].
- [[cross-platform | Cross-platform Development]].
- [[gui | GUI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries](https://github.com/john-cd/rust_howto/issues/1902)
</div>
