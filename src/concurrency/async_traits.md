# Async traits

As of Rust 1.75, it is possible to have `async` functions in traits:

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/async_traits.rs}}
```

[Stabilizing async fn in traits in 2023]( https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html )

This is in turn enabled by return-position `impl Trait` in traits, since `async fn` is sugar for functions that return `-> impl Future`.

```rust,editable
{{#include ../../deps/examples/async_traits2.rs}}
```

Note that there are still caveats for public traits - see [Announcing `async fn` and return-position `impl Trait` in traits]( https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html# ).

In addition, traits that use `-> impl Trait` and `async fn` are not object-safe, which means they lack support for dynamic dispatch. In the meanwhile, use the [Async trait crate (github)][async-trait-github].

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/async_traits3.rs}}
```

{{#include ../links.md}}
