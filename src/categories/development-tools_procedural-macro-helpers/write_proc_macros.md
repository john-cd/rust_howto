
## Parse Rust source code

[![syn][c-syn-badge]][c-syn]{{hi:syn}}
[![syn-crates.io][c-syn-crates.io-badge]][c-syn-crates.io]
[![syn-github][c-syn-github-badge]][c-syn-github]
[![syn-lib.rs][c-syn-lib.rs-badge]][c-syn-lib.rs]

[`syn`][c-syn]{{hi:syn}}⮳ is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

## `paste`

[![paste][c-paste-badge]][c-paste]{{hi:paste}}{{hi:Macros}}
[![paste-crates.io][c-paste-crates.io-badge]][c-paste-crates.io]
[![paste-github][c-paste-github-badge]][c-paste-github]
[![paste-lib.rs][c-paste-lib.rs-badge]][c-paste-lib.rs]

[`paste`][c-paste]⮳ provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

## `quote`

[![quote][c-quote-badge]][c-quote]{{hi:quote}}
[![quote-crates.io][c-quote-crates.io-badge]][c-quote-crates.io]
[![quote-github][c-quote-github-badge]][c-quote-github]
[![quote-lib.rs][c-quote-lib.rs-badge]][c-quote-lib.rs]

[`quote`][c-quote]⮳ provides the `quote!` macro for turning Rust syntax tree{{hi:Syntax tree}} data structures into tokens{{hi:Tokens}} of source code.

## `proc_macro2`

[![proc-macro2][c-proc_macro2-badge]][c-proc_macro2]{{hi:proc-macro2}}
[![proc-macro2-crates.io][c-proc_macro2-crates.io-badge]][c-proc_macro2-crates.io]
[![proc-macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github]
[![proc-macro2-lib.rs][c-proc_macro2-lib.rs-badge]][c-proc_macro2-lib.rs]
[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

[`proc_macro2`][c-proc_macro2]{{hi:proc-macro2}}⮳ [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

A substitute implementation of the compiler's `proc_macro` API to decouple token-based libraries from the procedural macro use case.

A wrapper around the procedural macro API of the compiler's proc_macro crate. This library serves two purposes:

Bring proc-macro-like functionality to other contexts like build.rs and main.rs. Types from proc_macro are entirely specific to procedural macros and cannot ever exist in code outside of a procedural macro. Meanwhile proc_macro2 types may exist anywhere including non-macro code. By developing foundational libraries like syn and quote against proc_macro2 rather than proc_macro, the procedural macro ecosystem becomes easily applicable to many other use cases and we avoid reimplementing non-macro equivalents of those libraries.

Make procedural macros unit testable. As a consequence of being specific to procedural macros, nothing that uses proc_macro can be executed from a unit test. In order for helper libraries or components of a macro to be testable in isolation, they must be implemented using proc_macro2.

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

### darling

[![darling][c-darling-badge]][c-darling]{{hi:darling}}
[![darling-crates.io][c-darling-crates.io-badge]][c-darling-crates.io]
[![darling-github][c-darling-github-badge]][c-darling-github]
[![darling-lib.rs][c-darling-lib.rs-badge]][c-darling-lib.rs]

Derive macro to easily parse derive macro inputs

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: write
</div>
