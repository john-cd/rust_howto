# Dynamic Typing

{{#include any.incl.md}}

## The `Any` Trait {#any-trait}

[std::any](https://doc.rust-lang.org/std/any/index.html)⮳{{hi:any}} provides functionality for working with dynamic types. It allows you to check the type of a value at runtime and perform operations based on that type.

The [`std::any::Any``](https://doc.rust-lang.org/std/any/trait.Any.html)⮳ trait is the core of this module. It is a special built-in trait that determines the concrete type of a trait object at runtime and provides the ability to downcast a trait object to a concrete type, given that the concrete type is known.

Downcasting allows you to convert a reference of a type to a reference of a more specific type if the value is of that type.

The `Any` trait provides a few primary methods:

- `is::<T>(&self) -> bool` checks if the underlying concrete type of the trait object is `T`.
- `downcast_ref::<T>(&self) -> Option<&T>` attempts to downcast the trait object to a reference of type `T`. Returns `Some(&T)` if successful, `None` otherwise.
- `downcast_mut::<T>(&mut self) -> Option<&mut T>` is similar to `downcast_ref`, but provides a mutable reference.
- `downcast::<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>` consumes a `Box<dyn Any>` and attempts to downcast it to a `Box<T>`.

`Any` is primarily used in scenarios where you need to store and retrieve data of various, unknown concrete types within a collection or context that uses trait objects (specifically `dyn Any`). This often comes up in:

- Plugins and Extensibility: If you're building a system where users can register custom data or components, and you need to store them polymorphically, Any allows you to later retrieve them and process them based on their original concrete type.
- Generic Event Systems: In an event system where different types of events might be processed by a single handler, Any can help identify and process specific event types.
- Error Handling (less common now): Historically, Box<dyn Any> was sometimes used for custom error types before std::error::Error became more prevalent and flexible.
- Reflection-like capabilities: While Rust doesn't have full reflection like some other languages, Any provides a limited form of runtime type inspection.

Caveats:

- `Any` requires `'static` lifetime: The `Any` trait can only be used with types that have a `'static` lifetime. This means they must not contain any non-'static references.
- Runtime Overhead: Downcasting involves runtime checks (`is::<T>`). While not excessively slow, it's less performant than static dispatch.
- Loss of Compile-Time Guarantees: When you put something into `Box<dyn Any>`, you lose type information at compile time. This makes it harder for the compiler to catch errors for you.
- Avoid if possible: If there's a way to achieve your goal using generics with trait bounds, enums, or other more type-safe Rust patterns, those are generally preferred over Any. Any should be used when you genuinely need runtime type introspection for heterogeneous collections.

## Dynamic Types {#dynamic-types}

[![std][c~std~docs~badge]][c~std~docs]

```rust,editable
{{#include ../../crates/standard_library/examples/any/any.rs:example}}
```

## Reflection {#reflection}

[![std][c~std~docs~badge]][c~std~docs]

Reflection in Rust is limited compared to other languages, but the `std::any` module provides a way to obtain type information at runtime. The [`TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html)⮳{{hi:std::any::TypeId}} struct represents a unique identifier for a type, allowing you to compare types without needing to know their names at compile time.

You can use `TypeId` to check if two types are the same or to store type information in a way that can be queried later.

```rust,editable
{{#include ../../crates/standard_library/examples/any/type_id.rs:example}}
```

## Plugins {#plugins}

[![std][c~std~docs~badge]][c~std~docs]

```rust,editable
{{#include ../../crates/standard_library/examples/any/plugin.rs:example}}
```

For a true plugin architecture where plugins are compiled as separate shared libraries (.so on Linux, .dll on Windows, .dylib on macOS) and loaded at runtime, you would typically:

- Use `libloading` crate: This crate provides safe FFI (Foreign Function Interface) wrappers to dynamically load shared libraries and resolve symbols (functions).
- Define a C-compatible ABI: Because Rust's internal ABI is not stable across different compiler versions or even minor changes, you should define your plugin interface using `#[repr(C)]` structs and `extern "C"` functions.
- Entry Point: Each plugin .so/.dll would export a specific extern "C" function (e.g., plugin_entry) that the host calls to get a Box<dyn Plugin>.
- Version Management: Even with `extern "C"`, you need robust versioning for your common plugin interface crate to prevent issues if host and plugins are compiled with different versions of the interface. Crates like `abi_stable` can help with this by providing more robust ABI compatibility checks.

## Related Topics {#related-topics}

- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]]
- [[scripting | Scripting]]
- [[traits | Traits]].

## References {#references}

- [`std::any`](https://doc.rust-lang.org/std/any/index.html)⮳{{hi:std::any}}.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
