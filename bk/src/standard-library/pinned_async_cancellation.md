<!-- TODO Reviewed ??-??-2026. -->

# Pin Futures and Own Their Cancellation

An `async fn` returns a future that does no work until it is polled. A future that may be `!Unpin` must be pinned before `select!` polls it by mutable reference. For a future used within one scope, `tokio::pin!` pins it on the stack. `Box::pin` is useful when the future must be owned behind a pointer or stored in a collection, at the cost of an allocation. See the [`Pin` documentation](https://doc.rust-lang.org/std/pin/) and [Tokio's `select!` documentation](https://docs.rs/tokio/latest/tokio/macro.select.html).

The first function below races a pending future against one deterministic yield. Leaving its scope drops the future, and a one-shot channel observes that its guard was dropped. The second function joins one task and aborts another; dropping a Tokio `JoinHandle` alone would detach its task.

```rust
{{#include ../../crates/cats/asynchronous/examples/pinned_cancellation.rs:example}}
```

Run `cargo test -p asynchronous --example pinned_cancellation` from `bk/crates`. Cancellation stops work at a suspension point; it cannot undo side effects that already happened. For cooperative shutdown, see the [Cancellation and Structured Concurrency](../categories/asynchronous/structured_concurrency.md) and its `CancellationToken` and `JoinSet` examples.

To spawn on Tokio's multi-thread runtime, the future must be `Send` across every `.await`. This deliberately failing example holds an `Rc` across a suspension point:

```rust,compile_fail,noplayground
{{#include ../../crates/language/tests/ui/future_holds_non_send_value.rs:example}}
```

Moving the `Rc` use entirely before `.await`, or using an owned `Send` value such as `Arc` when sharing across threads is necessary, changes that requirement. See the [staging Send/Sync guide](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/concurrency/send_sync.md). Actor frameworks such as [Tellus](https://docs.rs/tellus/latest/tellus/) also make mailbox ownership explicit, but do not replace task cancellation and join decisions.
