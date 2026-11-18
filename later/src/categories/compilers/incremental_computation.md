<!-- TODO Reviewed ??-??-2026. -->

# Incremental Computation

Incremental computation is a software engineering technique that attempts to save time by only recomputing those outputs which depend on changed data. When a program processes a large dataset and only a small portion of it changes, re-running the entire computation can be inefficient. Instead, an incremental computation framework keeps track of the dependencies between inputs and outputs. When an input changes, it only invalidates and recomputes the specific outputs affected by that change, reusing the previously computed results for the rest.

This paradigm is especially useful in domains such as:

- **Compilers**: Recompiling only the files or functions that have changed, rather than the whole codebase.
- **Build Systems**: Re-running only the build steps whose source files have been modified.
- **Interactive Applications**: Updating the user interface in response to user actions without re-rendering the entire screen.
- **Data Analysis Pipelines**: Updating metrics or dashboards when new data arrives without processing all historical data from scratch.

Rust offers several frameworks and libraries to facilitate incremental computation, each with its own approach to dependency tracking and memoization.

{{#include incremental_computation.incl.md}}

## Implementing Incremental Computation with `salsa` {#salsa}

{{#badges salsa development-tools=}}

[Salsa][c~salsa~website]↗{{hi:salsa}} is a Rust framework for writing incremental, on-demand programs - these are programs that want to adapt to changes in their inputs, continuously producing a new output that is up-to-date.

[`salsa`][c~salsa~docs]↗{{hi:salsa}} is designed for situations where you have a large computation that can be broken down into smaller, interdependent pieces. Salsa automatically tracks dependencies between these pieces, and when a change occurs, it only recomputes the affected parts, rather than the entire computation. This is crucial for [performance][p~performance] in scenarios like compilers, build systems, and interactive tools where changes are frequent and full recomputation is expensive.

```rust,editable
{{#include ../../../crates/cats/compilers/examples/incremental_computation/salsa.rs:example}}
```

## Implementing Incremental Computation with `comemo` {#comemo}

{{#badges comemo caching=Caching}}{{hi:Tracked}}{{hi:Incremental}}{{hi:Constraints}}{{hi:Memoization}}

[`comemo`][c~comemo~docs]↗{{hi:comemo}} offers incremental computation through constrained memoization.

[`comemo`][c~comemo~docs]↗{{hi:comemo}} performs runtime incremental computation through constrained memoization. A memoized function can receive a `Tracked<T>` value; methods marked with `#[track]` record which parts of that value the function observes. If unrelated parts change, a cached result can still be reused. The example changes an unobserved flag and then a tracked value to show the difference.

Use this approach when the computation can be expressed as deterministic functions over explicit inputs and tracked accesses. Do not rely on hidden mutable state or untracked I/O to affect a memoized result: the cache cannot invalidate work for dependencies it does not observe. Unlike `salsa`, which organizes work as a query database, `comemo` adds fine-grained tracking to functions. It is not compile-time evaluation; use `const fn` and const evaluation when a value must be computed during compilation.

```rust,editable
{{#include ../../../crates/cats/compilers/examples/incremental_computation/comemo.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [Development Tools: Cargo Plugins](https://github.com/john-cd/rust_howto/blob/develop/bk/src/categories/development-tools_cargo-plugins/index.md).
- [[parsing | Parsing]].
- [[parser-implementations | Parser Implementations]].
  - [[programming_languages | Programming Languages]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
