# Memory Address Pinning

{{#include pin.incl.md}}

## Give Data a Stable Memory Address with `Pin` {#pinned-data}

[![std][c~std~docs~badge]][c~std~docs]

The following is an advanced topic. It is highly recommended to read the [documentation](https://doc.rust-lang.org/std/pin/index.html)↗ in addition to the following summary.

In Rust, the address at which a value is located is not necessarily stable in-between borrows. The compiler is allowed to move a value to a new memory address in many places, for example during assignment or when passing a value into a function.

It is sometimes useful to be able to rely upon memory addresses not changing, especially when there are pointers pointing at that value.

The [`std::pin`](https://doc.rust-lang.org/std/pin/index.html)↗{{hi:std::pin}} module enables pinning data - preventing it from being moved in memory, and, more generally, guaranteeing it remains valid at that same memory location.

Pinning is typically used for self-referential types, compiler-generated generators for `async fn`, and intrusive data structure, where moving would break safety invariants:

- In _asynchronous programming_, you may have futures that need to maintain a stable memory address, as they may need to reference themselves or other data that should not be moved.
- Pinning a value is an useful building block for _unsafe code_ to be able to reason about whether a raw pointer to the pinned value is still valid.

The main type in `std::pin` is [`Pin<Ptr>`](https://doc.rust-lang.org/std/pin/struct.Pin.html)↗{{hi:std::pin::Pin}}, a smart pointer wrapper that flags that the data behind the pointer `Ptr` should not be moved in memory, even if it is mutable, _unless it implements `Unpin`_(see below).

Note that the pointer wrapped by `Pin` is not the value which we want to pin itself, but rather a _pointer_ to that value. A `Pin<Ptr>` does not pin the `Ptr`; instead, it pins the pointer's pointee value.

It is important to note that pinning does not make use of any compiler "magic". It _does not change the way the compiler behaves_ towards the inner value (it still considers the inner value fundamentally movable). Instead, `Pin<Ptr>` is a wrapper that prohibits calling code that would perform a move on the pinned value and enforces the use of `unsafe` code for dangerous operations. BEWARE: It is the responsibility of the programmer to implement that `unsafe` code correctly to satisfy the `Pin` invariants.

Because `Pin` has a restrictive API, Rust provide an "escape hatch" for the vast majority of Rust types, which have no address-sensitive states.

The [`Unpin`](https://doc.rust-lang.org/std/pin/trait.Unpin.html)↗{{hi:std::pin::Unpin}} trait is a built-in, auto-implemented marker trait that signifies a type can be safely moved in memory, even after it has been "pinned." It cancels the restrictive effects of `Pin`.

- Most types implement `Unpin` by default. Builtin types that are `Unpin` include all of the primitive types, like `bool`, `i32`, and `f32`, references (`&T` and `&mut T`), etc., as well as many core and standard library types like `Box<T>`, `String`.
- The compiler automatically implements `Unpin` for any `struct` or `enum` if _all_ of its fields are also `Unpin`.

Therefore, to define custom types that must not move in memory, you must "opt out" of the `Unpin` trait by having a field that is `!Unpin`. Rust provides for this purpose the marker type [`std::marker::PhantomPinned`](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html)↗{{hi:std::marker::PhantomPinned}}, which is `!Unpin`.

The following example demonstrates the basic usage of `Pin`, `Unpin`, `PhantomPinned` and the `pin!` macro:

```rust,editable
{{#include ../../crates/standard_library/examples/pin/pin.rs:example}}
```

## Related Topics {#related-topics}

- [[async_programming | Async Programming]]
- [[futures | Futures]].
- [[traits | Traits]].

## Related Data Structures {#related-data-structures}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
