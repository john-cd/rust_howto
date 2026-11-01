<!-- TODO Reviewed ??-??-2026 -->

# Async Traits {#async-traits}

{{#include async_traits.incl.md}}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

As of Rust 1.75, it is possible to have [`async`][book~rust-reference~async]↗{{hi:async}} functions in traits:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/async_traits.rs:example}}
```

[Stabilizing [async][p~async] fn in traits in 2023][blog~stabilizing-async-fn-in-traits]↗.

This is in turn enabled by return-position [`impl Trait`][book~rust-reference~impl-trait]↗{{hi:impl Trait}} in [traits][p~traits]{{hi:Return-position `impl Trait` in traits}}, since `async fn`{{hi:async fn}} is sugar for functions that return `-> impl Future`.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/async_traits2.rs:example}}
```

Note that there are still caveats for public [traits][p~traits] - see [Announcing `[async][p~async] fn` and return-position `impl Trait` in [traits][p~traits]][blog~announcing-async-fn]↗.

Traits that use `-> impl Trait` or `[async][p~async] fn` are not object-safe{{hi:Object-safe traits}}, so they cannot be used directly for dynamic dispatch. The [`async-trait`][c~async-trait~docs]↗{{hi:async-trait}} crate remains useful for dynamic dispatch and projects that support a Rust version earlier than 1.75.

[![async-trait][c~async-trait~docs~badge]][c~async-trait~docs]{{hi:async-trait}} [![async-trait~repo][c~async-trait~repo~badge]][c~async-trait~repo] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/async_traits3.rs:example}}
```

## Call a Native Async Trait Through Generics {#afit}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A trait with a native `async fn`{{hi:AFIT}}{{hi:Async fn in traits}} is used through generics (static dispatch). The compiler does not require the future to be [`Send`][c~std::marker::Send~docs]↗{{hi:Send}}, which suits single-threaded and local executors, but a generic task that you hand to `tokio::spawn` needs a `Send` future - see the next recipe.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/afit.rs:example}}
```

## Make the Futures of a Trait `Send` {#rpitit-send}

{{#badges trait-variant asynchronous=Asynchronous}}{{hi:Send}}{{hi:Return-position impl Trait in traits}}{{hi:RPITIT}}

`tokio::spawn` requires the future to be `Send`, and the compiler cannot know that for a generic `async fn` in a trait. There are two fixes:

- Declare the method as `fn f(&self) -> impl Future<Output = T> + Send`. Implementors may still write `async fn`; the compiler checks the bound.
- Annotate the trait with [`#[trait_variant::make(SendStore: Send)]`][c~trait-variant~docs]↗{{hi:trait_variant}}, which generates a second trait whose futures are `Send`. Keep the original trait for local executors.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/rpitit_send.rs:example}}
```

## Use Async Traits as Trait Objects {#async-trait-dyn}

[![async-trait][c~async-trait~docs~badge]][c~async-trait~docs] [![async-trait~repo][c~async-trait~repo~badge]][c~async-trait~repo]{{hi:async-trait}} [![dynosaur][c~dynosaur~docs~badge]][c~dynosaur~docs] [![dynosaur~repo][c~dynosaur~repo~badge]][c~dynosaur~repo]{{hi:dynosaur}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A trait with a native `async fn` is not dyn-compatible, so `Vec<Box<dyn Store>>` does not compile. [`async-trait`][c~async-trait~docs]↗{{hi:async-trait}} rewrites each method to return a `Pin<Box<dyn Future + Send>>`, at the cost of one heap allocation per call. [`dynosaur`][c~dynosaur~docs]↗{{hi:dynosaur}} is a newer alternative that keeps the native syntax.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_traits/async_trait_dyn.rs:example}}
```

## Choose a Style {#choose-a-style}

| Need                                              | Use                                                     |
| ------------------------------------------------- | ------------------------------------------------------- |
| Static dispatch, one thread or a local executor   | native `async fn` in the trait                          |
| Static dispatch, work spawned with `tokio::spawn` | `-> impl Future<Output = T> + Send`, or `trait_variant` |
| `dyn Trait`, for example a `Vec<Box<dyn Store>>`  | `#[async_trait]` (boxes each future) or `dynosaur`      |
| Support for a Rust version earlier than 1.75      | `#[async_trait]`                                        |

## Related Topics {#related-topics .skip}

- [[trait_objects | Trait Objects]].
- [[traits | Traits]].
- [[structured_concurrency | Cancellation and Structured Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  async-traits: review new Rust features
  async-trait = Provides a workaround for the lack of [language][p~language] support for [async][p~async] [functions][p~functions] in traits.
</div>
