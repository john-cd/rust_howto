# Dynamic Typing

{{#include dynamic_typing.incl.md}}

## Determine the Type of an Object at Runtime and Downcast with the `Any` Trait {#any-trait}

[![std][c~std~docs~badge]][c~std~docs]

Rust does not provide general-purpose runtime reflection. For concrete types that satisfy `'static`, the [`std::any`][c~std::any~docs]↗{{hi:std::any}} module provides limited runtime type identification and downcasting through [`TypeId`][c~std::any::TypeId~docs]↗ and [`Any`][c~std::any::Any~docs]↗. These APIs compare a value's concrete type; they do not inspect its fields or determine whether it implements an arbitrary trait. Prefer generics, enums, or trait objects when they can express the set of types your program expects.

`TypeId` is an opaque identifier for a type. Use it to compare types or as an in-process key, not as a type name or a durable identifier. Its hash and ordering can change between Rust releases, so do not persist them or rely on their order.

The example compares two types and obtains the identifier of a value:

```rust,editable
{{#include ../../crates/standard_library/examples/any/type_id.rs:example}}
```

The [`std::any::Any`][c~std::any::Any~docs]↗{{hi:std::any::Any}} trait supports type checks and safe downcasts from a `dyn Any` trait object. A borrowed trait object can be checked with [`is::<T>()`][c~std::any::Any::is~docs]↗ and downcast with `downcast_ref::<T>()` or `downcast_mut::<T>()`. A `Box<dyn Any>` can be consumed with `downcast::<T>()`. A failed downcast is reported as `false`, `None`, or `Err`, depending on the method; it does not guess or coerce to a related type.

```rust,editable
{{#include ../../crates/standard_library/examples/any/any.rs:example}}
```

Keep these limits in mind when using `Any`:

- `Any` is implemented for types that satisfy `'static`, which excludes a type containing a borrowed reference with a shorter lifetime. This is a bound on the concrete type; a value of that type can still be dropped normally.
- `Any` checks exact concrete type identity. It cannot check whether a value implements some other trait.
- Storing values behind `dyn Any` erases their concrete type from the static interface. Code must handle a failed check or downcast, and the runtime type check has a cost.
- Prefer a generic bound, enum, or domain-specific trait when it expresses the required behavior. Use `Any` when a heterogeneous collection or registry must recover a concrete type at runtime.

## Compile-time Type Inspection in Procedural Macros {#compile-type-reflection}

[![reflect][c~reflect~docs~badge]][c~reflect~docs] [![reflect~crates.io][c~reflect~crates.io~badge]][c~reflect~crates.io] [![reflect~repo][c~reflect~repo~badge]][c~reflect~repo] [![reflect~lib.rs][c~reflect~lib.rs~badge]][c~reflect~lib.rs]{{hi:reflect}}{{hi:Reflection}} [![cat~development-tools::procedural-macro-helpers][cat~development-tools::procedural-macro-helpers~badge]][cat~development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

The [`reflect`][c~reflect~docs]↗ crate explores a reflection-like API for writing derive macros. Procedural macros operate on Rust syntax during compilation and generate Rust code; the generated program does not gain general runtime reflection automatically ([Rust Reference][book~rust-reference~procedural-macros]). This crate is an experiment, not a standard-library facility or a replacement for runtime reflection in other languages.

## Related Topics {#related-topics .skip}

- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[scripting | Scripting]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
