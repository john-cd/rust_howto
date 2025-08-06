# Dynamic Typing

{{#include dynamic_typing.incl.md}}

## Determine the Type of an Object at Runtime and Downcast it with the `Any` Trait {#any-trait}

[![std][c~std~docs~badge]][c~std~docs]

Dynamic typing in Rust is limited compared to other languages, but the [`std::any`](https://doc.rust-lang.org/std/any/index.html)↗{{hi:std::any}} module provides ways to obtain type information at runtime.

`std::any` is primarily used in scenarios where you need to store and retrieve data of various, unknown concrete types within a collection or context that uses trait objects (specifically `dyn Any`{{hi:dyn Any}}). This often comes up in:

- Plugins: If you're building a system where users can register custom data or components, and you need to store them polymorphically, `Any` allows you to later retrieve them and process them based on their original concrete type.
- Generic Event Systems: In an event system where different types of events might be processed by a single handler, `Any` can help identify and process specific event types.
- Reflection-like capabilities: While Rust doesn't have full reflection like some other languages, `Any` provides a limited form of runtime type inspection.

The [`TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html)↗{{hi:std::any::TypeId}} struct represents a globally unique, opaque identifier for a type, allowing you to compare types without needing to know their names at compile time. A `TypeId` is currently only available for types which are `'static`.

You can use `TypeId` to check if two types are the same or as a key to store type information in e.g. a `HashMap`:

```rust,editable
{{#include ../../crates/standard_library/examples/any/type_id.rs:example}}
```

The [`std::any::Any`](https://doc.rust-lang.org/std/any/trait.Any.html)↗{{hi:std::any::Any}} trait is the core of the `std::any` module. It is a special built-in trait that determines the concrete type of a trait object _at runtime_ and provides the ability to _downcast_ a trait object to a concrete type, when the concrete type is known ("downcasting" converts a reference of a general type (here `dyn Any` and related types) to a reference of a more specific type, if the value is of that type). The `Any` trait provides a few methods:

```rust,editable
{{#include ../../crates/standard_library/examples/any/any.rs:example}}
```

Note a few caveats with `Any`:

- The `Any` trait can only be used with types that have a `'static` lifetime. This means they must not contain any non-`'static` references.
- Runtime Overhead: Downcasting involves runtime checks (`is::<T>`). While not excessively slow, it's less performant than static dispatch.
- Loss of Compile-Time Guarantees: When you put something into `Box<dyn Any>` or similar, you lose type information at compile time. This makes it harder for the compiler to catch errors for you.
- If there's a way to achieve your goal using generics with trait bounds, enums, or other more type-safe Rust patterns, those are generally preferred over `Any`. `Any` should be used when you genuinely need runtime type introspection for heterogeneous collections.

## Related Topics {#related-topics}

- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[scripting | Scripting]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
