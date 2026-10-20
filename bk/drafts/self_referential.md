# Self-Referential Types

{{#include self_referential.incl.md}}

Self-referential types hold references to memory owned by the same struct. This is tricky, because moving the struct changes the address of its fields, which can invalidate internal references.

## Create a Self-Referential Type using `Pin` {#self-referential-type}

[![std][c~std~docs~badge]][c~std~docs]

This is an advanced topic. A self-referential struct must be pinned after construction to guarantee that it never moves again.

```rust,editable
{{#include ../../crates/language/examples/lifetimes/self_referential_struct2.rs:example}}
```

### Why this is hard

- Rust values can normally be moved freely, including when they are passed by value or stored in collections.
- A self-reference must point to stable memory for the lifetime of the owning value.
- `Pin` provides the guarantee that a value will remain in place, which is the key requirement for safe self-referential structs.

### When to use self-referential types

Only use this pattern when you truly need a stable reference to data owned by the same object. In many cases, a heap-allocated container such as `Box`, `Rc`, or `Arc`, or a design that avoids self-references entirely, is easier and safer.

## See Also {#see-also .skip}

[![movable-ref][c~movable-ref~docs~badge]][c~movable-ref~docs] [![movable-ref~crates.io][c~movable-ref~crates.io~badge]][c~movable-ref~crates.io] [![movable-ref~repo][c~movable-ref~repo~badge]][c~movable-ref~repo] [![movable-ref~lib.rs][c~movable-ref~lib.rs~badge]][c~movable-ref~lib.rs]{{hi:movable-ref}}{{hi:Offset}}{{hi:Pointer}}{{hi:No-std}}{{hi:Self-referential}}{{hi:Movable}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

A tool for building movable self-referential types.

## Related Topics {#related-topics .skip}

- [[ownership_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
