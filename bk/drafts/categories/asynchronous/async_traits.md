# Async Traits {#async-traits}

{{#include async_traits.incl.md}}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

As of Rust 1.75, it is possible to have [`async`][book~rust-reference~async]{{hi:async}}↗ functions in traits:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async-traits/async_traits.rs:example}}
```

[Stabilizing [async][p~async] fn in traits in 2023][blog~stabilizing-async-fn-in-traits]↗.

This is in turn enabled by return-position `impl Trait` in [traits][p~traits]{{hi:Return-position `impl Trait` in traits}}, since `async fn`{{hi:async fn}} is sugar for functions that return `-> impl Future`.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async-traits/async_traits2.rs:example}}
```

Note that there are still caveats for public [traits][p~traits] - see [Announcing `[async][p~async] fn` and return-position `impl Trait` in [traits][p~traits]][blog~announcing-async-fn]↗.

In addition, [traits][p~traits] that use `-> impl Trait` and `[async][p~async] fn` are not object-safe{{hi:Object-safe traits}}, which means they lack support for dynamic dispatch. In the meanwhile, use the [`async-trait`][c~async-trait~docs]↗{{hi:async-trait}} crate.

[![async-trait][c~async-trait~docs~badge]][c~async-trait~docs]{{hi:async-trait}} [![async-trait~github][c~async-trait~github~badge]][c~async-trait~github] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async-traits/async_traits3.rs:example}}
```

## Related Topics {#related-topics}

- [[trait_objects | Trait Objects]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[async-traits: review new Rust features](https://github.com/john-cd/rust_howto/issues/216)
async-trait = Provides a workaround for the lack of [language][p~language] support for [async][p~async] [functions][p~functions] in traits
</div>
