# Incremental Computation

{{#include incremental_computation.incl.md}}

## `salsa` {#salsa}

[![salsa][c-salsa-badge]][c-salsa] [![salsa-crates.io][c-salsa-crates.io-badge]][c-salsa-crates.io] [![salsa-github][c-salsa-github-badge]][c-salsa-github] [![salsa-lib.rs][c-salsa-lib.rs-badge]][c-salsa-lib.rs]{{hi:salsa}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[Salsa][c-salsa-website]{{hi:salsa}}⮳ [(GitHub)][c-salsa-github]⮳ is a framework for on-demand, incremental computation.

[`salsa`][c-salsa]⮳{{hi:salsa}} is a Rust framework for writing incremental, on-demand programs -- these are programs that want to adapt to changes in their inputs, continuously producing a new output that is up-to-date.

`salsa` is designed for situations where you have a large computation that can be broken down into smaller, interdependent pieces.  Salsa automatically tracks dependencies between these pieces, and when a change occurs, it only recomputes the affected parts, rather than the entire computation.  This is crucial for performance in scenarios like compilers, build systems, and interactive tools where changes are frequent and full recomputation is expensive.

## `comemo` {#comemo}

[![comemo][c-comemo-badge]][c-comemo] [![comemo-crates.io][c-comemo-crates.io-badge]][c-comemo-crates.io] [![comemo-github][c-comemo-github-badge]][c-comemo-github] [![comemo-lib.rs][c-comemo-lib.rs-badge]][c-comemo-lib.rs]{{hi:comemo}}{{hi:Tracked}}{{hi:Incremental}}{{hi:Constraints}}{{hi:Memoization}} [![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}

[`comemo`][c-comemo]⮳{{hi:comemo}} offers incremental computation through constrained memoization.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[incremental_computation: write / add to index.md and summary (P2)](https://github.com/john-cd/rust_howto/issues/244)

`comemo` provides memoization utilities, primarily focusing on compile-time memoization.  It allows you to precompute the results of functions at compile time and embed them directly into the binary.  This can significantly improve runtime performance for functions with expensive computations but fixed inputs. comemo uses const generics and procedural macros to achieve this compile-time evaluation.  It's particularly useful for lookups, precomputed tables, or any situation where the function's inputs are known at compile time.  Unlike runtime memoization, comemo avoids any runtime overhead associated with checking and storing cached values.  However, it does increase compile time as the computations are performed during compilation.

- Cargo's Incremental Compilation: This is the primary mechanism for incremental compilation in Rust.  It's built into Cargo and generally "just works" automatically.

- Understanding Incremental Compilation:  Knowing how Cargo's incremental compiler works (caching, invalidation) is key to maximizing its benefits.  Changes to dependencies, function signatures, or certain code structures can invalidate the cache.

- Minimizing Cache Invalidation:  This is the main focus when trying to improve incremental compilation.  Strategies include:

- Keeping dependencies stable.
Structuring code to minimize changes that trigger recompilation (e.g., separating interface and implementation).
Being mindful of how generics and macros can affect recompilation.
Build Profiles: Release builds can sometimes behave differently with incremental compilation due to optimizations.

cargo tree is also a useful tool for dependency analysis.

- Debugging Incremental Compilation:  Sometimes, unexpected recompilation can occur.  Cargo's verbose output can help diagnose these situations.  There aren't specific crates for this, but careful observation of compiler output is key.

</div>
