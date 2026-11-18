<!-- TODO Reviewed ??-??-2026. -->

# Self-Referential Types

{{#include self_referential.incl.md}}

A self-referential value stores a pointer or reference into data owned by that same value. Moving it can invalidate that pointer. Rust's borrow checker prevents ordinary safe code from creating many such structures, which is why a design that avoids the self-reference is usually preferable.

## Prefer a Design Without a Self-Reference {#safe-alternatives}

The example contrasts an owned value whose would-be self-reference cannot be filled safely, a record that borrows from a string owned elsewhere, and two `Rc` handles that share one allocation. The latter designs make the ownership relationship explicit.

```rust,noplayground
{{#include ../../../crates/language/examples/lifetimes/self_referential_struct.rs:example}}
```

## Build a Pinned Self-Referential Type {#pinned-self-reference}

[`Pin`][c~std::pin::Pin~docs]↗ guarantees address stability only when the pointee is `!Unpin`. Pinning does not initialize a self-pointer or prove that it is valid. The example first pins a boxed value, then uses a small unsafe block to store a pointer to one of its fields. Its private fields and limited API preserve that invariant afterward.

```rust,noplayground
{{#include ../../../crates/language/examples/lifetimes/self_referential_struct2.rs:example}}
```

This construction depends on more than `Pin` alone:

- Pin the value before creating a pointer into it.
- Keep the type `!Unpin` and do not expose safe operations that move or replace its address-sensitive fields.
- Maintain the pointer invariant through every mutation and during destruction.
- Audit each unsafe operation; a bug in the invariant can make later pointer dereferences undefined behavior.

Moving a `Pin<Box<T>>` moves the owning handle while leaving its heap allocation in place. Pinning is useful for address-sensitive types, but a borrow, index, or shared owner is simpler when it expresses the same relationship.

## Related Topics {#related-topics .skip}

- [Memory Address Pinning](../../standard-library/memory_address_pinning.md).
- [Lifetimes](../../language/lifetimes.md).
- [Ownership and Borrowing](../../language/ownership_and_borrowing.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
