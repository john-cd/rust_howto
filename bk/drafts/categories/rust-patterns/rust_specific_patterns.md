<!-- TODO Reviewed ??-??-2026 -->

# Rust-specific Patterns

{{#include rust_specific_patterns.incl.md}}

## Clone a Struct Storing a Boxed Trait Object {#dyn-clone}

{{#badges dyn-clone rust-patterns="Rust patterns" no-std="No standard library"}}

The [`dyn-clone`][c~dyn-clone~docs]↗{{hi:dyn-clone}} crate provides a [`DynClone`][c~dyn-clone::DynClone~docs]↗{{hi:dyn-clone::DynClone}} trait that can be used in trait objects, and a [`clone_box`][c~clone_box~docs]↗{{hi:clone_box}} function that can clone any sized or dynamically sized implementation of `DynClone`. Types that implement the standard library's [`std::clone::Clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} trait are automatically usable by a `DynClone` trait object.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/dyn_clone.rs:example}}
```

## `pin-project` and `pin-project-lite` {#pin-project}

{{#badges pin-project no-std="No standard library" no-std::no-alloc="No dynamic allocation" rust-patterns="Rust patterns"}}{{hi:Attribute}}{{hi:Macros}}{{hi:Pin}}

{{#badges pin-project-lite rust-patterns="Rust patterns" no-std="No standard library" no-std::no-alloc="No dynamic allocation"}}{{hi:Macros}}{{hi:Pin}}

[`pin-project`][c~pin-project~docs]↗{{hi:pin-project}} is a crate for safe and ergonomic pin-projection. The `#[pin_project]` attribute creates projection types covering all the fields of struct or enum.

`pin-project-lite` is a lightweight version of `pin-project` written with declarative macros. The `pin_project!` macro creates a projection type covering all the fields of struct.

Future implementations can be `!Unpin`, so a wrapper must poll the inner future through a pinned projection. Other wrapper state, such as a poll counter, can remain movable.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs:example}}
```

See also:

- [Projections and Structural Pinning][c~std::pin~projections-and-structural-pinning~docs]↗.
- [`std::pin::Pin`][c~std::pin::Pin~docs]↗.

## Run Code on Scope Exit with `scopeguard` {#scopeguard}

{{#badges scopeguard rust-patterns="Rust patterns"}}

[`scopeguard`][c~scopeguard~docs]↗{{hi:scopeguard}} is a library that implements a scope guard: a type that runs a closure when it goes out of scope, even if code panics (as long as `panic` doesn't abort). This is useful for RAII-style resource management — releasing locks, closing file handles, reverting state changes, and similar cleanup tasks.

Use the `defer!` macro for fire-and-forget cleanup, or `scopeguard::guard` when the cleanup closure needs to operate on a specific owned value.

```rust,editable,noplayground
{{#include ../../../crates/cats/rust_patterns/examples/scopeguard.rs:example}}
```

See also [`std::ops::Drop`][c~std::ops::Drop~docs]↗{{hi:std::ops::Drop}} for custom cleanup on your own types.

## Domain Newtypes {#domain-newtypes}

A _newtype_{{hi:Newtype}} wraps a primitive in a type that carries meaning and, when needed, enforces an invariant. The compiler then rejects mixing a distance with a duration, or passing an unvalidated string where a validated email address is expected. For the pattern itself, see [Newtype][ex~rust-patterns~newtype].

### Newtypes That Carry a Unit {#newtype-units}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std::ops::Add}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Implement only the operations that make sense. `Meters` can be added to and subtracted from `Meters` and scaled by a plain `f64`, but there is no `Mul<Meters>`: multiplying two distances would not give a distance, so the compiler rejects it.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/newtype_units.rs:example}}
```

### A Validated Newtype {#validated-newtype}

{{#badges serde rust-patterns="Rust patterns"}}{{hi:Validation}}{{hi:TryFrom}}{{hi:FromStr}}

An `EmailAddress` that cannot hold invalid data has a private field and one way in: [`TryFrom`][c~std::convert::TryFrom~docs]↗{{hi:TryFrom}} and [`FromStr`][c~std::str::FromStr~docs]↗{{hi:FromStr}} validate their input. Adding `#[serde(try_from = "String", into = "String")]` makes deserialization go through the same validation, so a bad value in a JSON document is rejected too.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/validated_newtype.rs:example}}
```

[`derive_more`][c~derive_more~docs]↗{{hi:derive_more}} derives `From`, `Display`, `Add` and similar traits and removes the boilerplate for an unrestricted newtype. Write the impls by hand when they enforce an invariant. Avoid `Deref` on a type that enforces invariants: it lets callers bypass the checks. Add `#[repr(transparent)]` when the newtype must have the same layout as the wrapped type, for example at an FFI boundary.

### Equality That Ignores Presentation Fields {#newtype-equality}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std::hash::Hash}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

When a value has a field that only affects how it is shown, such as a display style, `PartialEq` and `Hash` should ignore it. The two must agree: values that compare equal must hash equally, so implement both by hand and use the same fields.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/newtype_equality.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[asynchronous | Asynchronous]]: `Pin` and `pin-project` are frequently used when manually implementing `Future`s and async state machines.
- [[resource_cleanup | Resource Cleanup]]: the `Drop` trait, which underpins RAII and `scopeguard`.
- [[smart_pointers | Smart Pointers]]: `Box<dyn Trait>` and pinned smart pointers.
- [[traits | Traits]]: trait objects and dynamic dispatch, including the newtype pattern.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
write

- [& vs. ref in Rust patterns][blog~rust-patterns-ref]↗.

</div>
