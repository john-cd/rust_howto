# Development Tools Written in Rust

{{#include development_tools.incl.md}}

## JavaScript Tooling {#javascript-tooling}

- [Volta][volta~website]↗[(github)][volta~github]↗.
- [Deno][deno~website]↗ uses Rust for its JavaScript and TypeScript runtime.
- [`Bun`][bun.sh~website]↗{{hi:Bun}}.

### `swc` Compiler {#swc}

[![swc][c~swc~docs~badge]][c~swc~docs]{{hi:swc}}
[![swc~crates.io][c~swc~crates.io~badge]][c~swc~crates.io]
[![swc~github][c~swc~github~badge]][c~swc~github]
[![swc~lib.rs][c~swc~lib.rs~badge]][c~swc~lib.rs]

[`swc` (github)][c~swc~github]↗ (stands for `Speedy Web Compiler`) is a fast TypeScript / JavaScript compiler written in Rust. It's a library for Rust and JavaScript at the same time.

See also [[programming_languages | Programming Language]] parsers.

## Code Editors {#code-editors}

### `lapce` {#lapce}

[![lapce~website][lapce~website~badge]][lapce~website]{{hi:lapce}}

[`lapce`][lapce~website]{{hi:lapce}}↗ is a lightning-fast and Powerful Code Editor written in Rust.

### `zed` {#zed}

{{hi:zed}}
[![zed~crates.io][c~zed~crates.io~badge]][c~zed~crates.io]
[![zed~github][c~zed~github~badge]][c~zed~github]
[![zed~lib.rs][c~zed~lib.rs~badge]][c~zed~lib.rs]

[`zed`][c~zed~github]{{hi:zed}}↗ is a high-performance, multiplayer code editor from the creators of Atom and Tree-sitter.

## Build Tools {#build-tools}

[`pantsbuild.org`][pantsbuild~website]↗ {{hi:pants}} is a fast, scalable, user-friendly build system for codebases of all sizes. It's currently focused on Python, Go, Java, Scala, Kotlin, Shell, and Docker.

## Utilities {#utilities}

[`television`][c~television~docs]↗{{hi:television}} is a blazingly fast general purpose fuzzy finder TUI.

It integrates with your shell and lets you quickly [search][p~search] through any kind of data source (files, git repositories, [environment variables][p~environment-variables], docker images, you name it) using a fuzzy matching algorithm.

## Related Topics {#related-topics}

- [[programming_languages | Programming Language]] parsers.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/612)

- [onefetch: Command-line Git information tool][c~onefetch~github]↗.

</div>
