# Incremental Computation

{{#include incremental_computation.incl.md}}

## Implementing Incremental Computation with `salsa` {#salsa}

[![salsa][c-salsa-badge]][c-salsa] [![salsa-crates.io][c-salsa-crates.io-badge]][c-salsa-crates.io] [![salsa-github][c-salsa-github-badge]][c-salsa-github] [![salsa-lib.rs][c-salsa-lib.rs-badge]][c-salsa-lib.rs]{{hi:salsa}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[Salsa][c-salsa-website]{{hi:salsa}}⮳ is a Rust framework for writing incremental, on-demand programs - these are programs that want to adapt to changes in their inputs, continuously producing a new output that is up-to-date.

[`salsa`][c-salsa]⮳{{hi:salsa}} is designed for situations where you have a large computation that can be broken down into smaller, interdependent pieces. Salsa automatically tracks dependencies between these pieces, and when a change occurs, it only recomputes the affected parts, rather than the entire computation. This is crucial for [performance][p-performance] in scenarios like compilers, build systems, and interactive tools where changes are frequent and full recomputation is expensive.

```rust,editable
{{#include ../../../crates/cats/compilers/examples/incremental_computation/salsa.rs:example}}
```

## Implementing Incremental Computation with `comemo` {#comemo}

[![comemo][c-comemo-badge]][c-comemo] [![comemo-crates.io][c-comemo-crates.io-badge]][c-comemo-crates.io] [![comemo-github][c-comemo-github-badge]][c-comemo-github] [![comemo-lib.rs][c-comemo-lib.rs-badge]][c-comemo-lib.rs]{{hi:comemo}}{{hi:Tracked}}{{hi:Incremental}}{{hi:Constraints}}{{hi:Memoization}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}

[`comemo`][c-comemo]⮳{{hi:comemo}} offers incremental computation through constrained memoization.

[`comemo`][c-comemo]⮳{{hi:comemo}} provides memoization utilities, primarily focusing on compile-time memoization. It allows you to precompute the results of [functions][p-functions] at compile time and embed them directly into the binary. This can significantly improve runtime [performance][p-performance] for functions with expensive computations but fixed inputs. [`comemo`][c-comemo]⮳{{hi:comemo}} uses `const` generics and procedural macros to achieve this compile-time evaluation. It's particularly useful for lookups, precomputed tables, or any situation where the function's inputs are known at compile time. Unlike runtime memoization, [`comemo`][c-comemo]⮳{{hi:comemo}} avoids any runtime overhead associated with checking and storing cached values. However, it does increase compile time as the computations are performed during compilation.

```rust,editable
{{#include ../../../crates/cats/compilers/examples/incremental_computation/comemo.rs:example}}
```

## Related Topics {#skip}

- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[parsing | Parsing]].
- [[parser-implementations | Parser Implementations]].
  - [[programming_languages | Programming Languages]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[incremental_computation: write](https://github.com/john-cd/rust_howto/issues/244)
</div>
