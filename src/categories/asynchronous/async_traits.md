# Async traits

{{#include async_traits.incl.md}}

As of Rust 1.75, it is possible to have [`async`][book-rust-reference-async]{{hi:async}}⮳ functions in traits:

```rust
{{#include ../../../deps/tests/async_traits.rs}}
```

[Stabilizing async fn in traits in 2023][blog-stabilizing-async-fn-in-traits]⮳

This is in turn enabled by return-position `impl Trait` in traits{{hi:Return-position `impl Trait` in traits}}, since `async fn`{{hi:async fn}} is sugar for functions that return `-> impl Future`.

```rust
{{#include ../../../deps/tests/async_traits2.rs}}
```

Note that there are still caveats for public traits - see [Announcing `async fn` and return-position `impl Trait` in traits][blog-announcing-async-fn]⮳.

In addition, traits that use `-> impl Trait` and `async fn` are not object-safe{{hi:Object-safe traits}}, which means they lack support for dynamic dispatch. In the meanwhile, use the `async-trait`{{hi:async-trait}} crate.

[![async-trait][c-async_trait-badge]][c-async_trait]{{hi:async-trait}}  [![async_trait-github][c-async_trait-github-badge]][c-async_trait-github]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust
{{#include ../../../deps/tests/async_traits3.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: review new Rust features

async-trait = Provides a workaround for the lack of language support for async functions in traits
</div>
