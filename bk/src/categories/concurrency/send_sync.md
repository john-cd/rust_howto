<!-- TODO Reviewed ??-??-2026. -->

# Send and Sync Traits

{{#include send_sync.incl.md}}

The [`std::marker::Send`][c~std::marker::Send~docs]↗{{hi:std::marker::Send}} and [`std::marker::Sync`][c~std::marker::Sync~docs]↗{{hi:std::marker::Sync}} [traits][p~traits] are fundamental to Rust's [concurrency][p~concurrency]. `Send` describes whether ownership can cross a thread boundary. `Sync` describes whether threads can safely share a reference.

`T` is `Sync` if and only if `&T` is `Send`.

## Existing Implementations of `Send` and `Sync` {#existing-impl-send-sync}

| Traits | Types |
| --- | --- |
| [`std::marker::Send`][c~std::marker::Send~docs]↗{{hi:std::marker::Send}} and [`std::marker::Sync`][c~std::marker::Sync~docs]↗{{hi:std::marker::Sync}} | primitives; `(T1, T2)↗`, `[T; N]`, `&[T]`, `struct { x: T }`, `Arc`, `Vec`, `Box`, [`Option`][c~std::option::Option~docs]↗{{hi:Option}} (depending on underlying types); [`String`][c~std::string::ToString~docs]↗{{hi:String}}, `&str`; `Mutex`, `Atomic*`... |
| `!Send` and `!Sync` | `Rc`, raw pointers `*const T`,`*mut T`, types from external libraries or the operating system that are not thread safe. |
| `Send` and `!Sync` | `mpsc::Receiver<T>`, `UnsafeCell<T>`, `Cell<T>`, and `RefCell<T>` when `T: Send`. Ownership can move to another thread. Sharing a reference across threads would bypass the exclusive access these types require. |
| `!Send` and `Sync` | A [`MutexGuard<T>`][c~std::sync::MutexGuard~docs]↗{{hi:MutexGuard}} can be shared by reference when `T: Sync`. The acquiring thread must drop the guard. |

## `Send` Trait {#send}

A type is [`std::marker::Send`][c~std::marker::Send~docs]↗{{hi:std::marker::Send}} if it can be _transferred across thread boundaries_. Most types in Rust are `Send` by default, as long as they don't contain non-`Send` types.

For example, sending a `String` through a channel moves the string to the receiving thread. The sender can no longer read it after the move.

`Rc` illustrates why `Send` matters. Cloning an `Rc` creates another owner of the same allocation. If one owner could move to another thread, both threads could update the same non-atomic reference count concurrently.

## `Sync` Trait {#sync}

A type is [`Sync`][c~std::marker::Sync~docs]↗{{hi:std::marker::Sync}} when multiple threads can safely hold shared references to it. For shared mutable state, a `Mutex` serializes access, an `RwLock` allows multiple readers or one writer, and [atomic types][p~atomic-types] support synchronized operations on individual values.

`Arc<T>` uses an atomic reference count. With the default allocator, it implements `Send` and `Sync` when `T` implements both traits. Use `Arc<Mutex<T>>` to share mutable state across threads when `T: Send`. The mutex grants exclusive access to its contents.

By utilizing [interior mutability][p~interior-mutability], types like `Mutex` and `RwLock` provide safe ways to modify data that is shared across threads.

## Implementing `Send` and `Sync` {#implementing-send-sync}

`Send` and `Sync` are automatically derived [traits][p~traits]. This means that, unlike almost every other trait, if a type is composed entirely of `Send` or `Sync` types, then it is `Send` or `Sync`.

Raw pointers implement neither `Send` nor [`Sync`][c~std::marker::Sync~docs]↗{{hi:std::marker::Sync}}. A wrapper can implement either trait with `unsafe impl` if its author can justify the corresponding thread-safety guarantee.

The following example provides a simplified implementation of [`std::sync::Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} (atomic reference counted). Each `my::Arc` owns one strong count on a shared `ArcInner` allocation. Cloning uses a checked atomic increment, and the final drop pairs its Release decrement with an Acquire fence before reconstructing the owning `Box`. That final `Box` drop destroys the value once and frees the allocation. The example is for studying unsafe ownership invariants, not for production use.

By implementing the unsafe marker [traits][p~traits] [`std::marker::Send`][c~std::marker::Send~docs]↗{{hi:std::marker::Send}} and `Sync` on `my::Arc`, the example asserts that it can be sent and shared across threads safely. The implementation must prevent data races and invalid memory access; otherwise it can cause undefined behavior. The included test moves the final owner to another thread and checks that the value is dropped once. Run the documented Miri command to check pointer validity, drop behavior, and allocation leaks for that execution; Miri does not model weak-memory reorderings.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/send_sync/send_sync_impl.rs:example}}
```

## References {#references .skip}

- [Understanding the `Send` trait][stackoverflow~understanding-the-send-trait]↗.
- [The Rustonomicon: `Send` and `Sync`][book~rustonomicon~send-and-sync]↗.
- [The Rustonomicon: Implementing `Vec`][book~rustonomicon~implementing-vec]↗.
- [An unsafe tour of Rust's `Send` and `Sync`][blog~an-unsafe-tour-of-rust-s-send-and-sync]↗.
- [Extensible Concurrency with the `Sync` and `Send` Traits][book~rust-extensible-concurrency-sync-and-send]↗.
- [Rust: A unique perspective][blog~rust-a-unique-perspective-html]↗.
- [References][primitive~reference]↗.
- [Pointers][primitive~pointer]↗.
- [Pointer functions][c~std::ptr~docs]↗.
- [NonNull pointer][c~std::ptr::NonNull~docs]↗.
- [std::fmt::Pointer][c~std::fmt::Pointer~docs]↗.
- [phantom-data][book~rustonomicon~phantom-data]↗.
- [PhantomData][c~std::marker::PhantomData~docs]↗.

## Related Topics {#related-topics .skip}

- [Shared-State Concurrency](shared_state.md).
- [Ownership and Borrowing](../../language/ownership_and_borrowing.md).
- [Smart Pointers](../../standard-library/smart_pointers.md).
- [Interior Mutability](../../standard-library/interior_mutability.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [Implementing `Vec`][book~rustonomicon~implementing-vec]↗.
- [An unsafe tour of Rust's Send and Sync][blog~an-unsafe-tour-of-rust-s-send-and-sync]↗.
- [Rust: A unique perspective][blog~rust-a-unique-perspective]↗.

REFERENCES:

- [References][primitive~reference]↗.

POINTERS:

- [Pointers][primitive~pointer]↗.
- [Pointer functions][c~std::ptr~docs]↗.
- [NonNull pointer][c~std::ptr::NonNull~docs]↗.
- [std::fmt::Pointer][c~std::fmt::Pointer~docs]↗.

SEND / SYNC:

- [The Rustonomicon: `Send` and `Sync`][book~rustonomicon~send-and-sync]↗.

PHANTOM DATA:

- [phantom-data][book~rustonomicon~phantom-data]↗.
- [PhantomData][c~std::marker::PhantomData~docs]↗.
- [Understanding the `Send` trait][stackoverflow~understanding-the-send-trait]↗.

</div>
