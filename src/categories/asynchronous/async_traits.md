# Async traits

{{#include async_traits.incl.md}}

As of Rust 1.75, it is possible to have `async` functions in traits:

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async_traits.rs}}
```

[Stabilizing async fn in traits in 2023][blog-stabilizing-async-fn-in-traits]⮳

This is in turn enabled by return-position `impl Trait` in traits, since `async fn` is sugar for functions that return `-> impl Future`.

```rust,editable
{{#include ../../../deps/tests/async_traits2.rs}}
```

Note that there are still caveats for public traits - see [Announcing `async fn` and return-position `impl Trait` in traits][blog-announcing-async-fn]⮳.

In addition, traits that use `-> impl Trait` and `async fn` are not object-safe, which means they lack support for dynamic dispatch. In the meanwhile, use the `async-trait` crate.

[![async-trait][async-trait-badge]][async-trait]  [![async-trait-github][async-trait-github-badge]][async-trait-github]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async_traits3.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
