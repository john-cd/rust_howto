# `std::pin`

{{#include pin.incl.md}}

## Give Data a Stable Memory Address with `Pin` {#pinned-data}

[![std][c~std~docs~badge]][c~std~docs]

The [`std::pin`](https://doc.rust-lang.org/std/pin/index.html)⮳{{hi:pin}} module works with pinned data - data that is prevented from being moved in memory, and, more generally, remain valid at that same memory location.

This is an advanced topic.

- Pinning is particularly useful for types with self-referential or address-sensitive internals, for example in asynchronous programming, where you may have futures that need to maintain a stable memory address, as they may need to reference themselves or other data that should not be moved.
- For example, when you create a future that references itself or other data, you need to pin it to ensure that it does not get moved while it is being polled. This is typically done using `Pin<Box<F>>`, where `F` is the type of the future.
- Use pinning (`Pin<Ptr>`) for self-referential types, compiler-generated generators for `async fn`, or intrusive data structure, where moving would break safety invariants.
- Pinning a value is an useful building block for unsafe code to be able to reason about whether a raw pointer to the pinned value is still valid.

- The [`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html)⮳ type is used to create pinned data. `Pin<Ptr>` is a smart pointer wrapper that guarantees the data behind the pointer `Ptr` will not be moved in memory, even if it is mutable, unless it implements `Unpin`.
- The [`Unpin`](https://doc.rust-lang.org/std/pin/trait.Unpin.html)⮳ trait is used to indicate that a type can be safely moved even if it is pinned. Most types in Rust implement `Unpin` by default.
- Builtin types that are Unpin include all of the primitive types, like bool, i32, and f32, references (&T and &mut T), etc., as well as many core and standard library types like Box<T>, String, and more.
- Types that must never move should not implement `Unpin` (written `!Unpin`).
- Rely on `Unpin` and regular pointers (`&mut T`, `Box<T>`) for types that don't care about their address stability, enjoying zero-cost moves and simple APIs.

Common smart-pointer types such as Box<T> and &mut T also allow moving the underlying value they point at: you can move out of a Box<T>, or you can use mem::replace to move a T out of a &mut T.

Notice that the thing wrapped by Pin is not the value which we want to pin itself, but rather a pointer to that value! A Pin<Ptr> does not pin the Ptr; instead, it pins the pointer's pointee value.

```rust,editable
{{#include ../../crates/standard_library/examples/pin/pin.rs:example}}
```

## Create a Self-Referential Type using `Pin` {#self-referential-type}

[![std][c~std~docs~badge]][c~std~docs]

This is an advanced topic.

```rust,editable
{{#include ../../crates/standard_library/examples/pin/pin2.rs:example}}
```

## Related Topics {#related-topics}

- [[async_programming | Async Programming]]
- [[futures | Futures]].
- [[traits | Traits]].

## Related Data Structures {#related-data-structures}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[finish writing](https://github.com/john-cd/rust_howto/issues/1396)
</div>
