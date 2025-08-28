# Generics

{{#include generics.incl.md}}

## Generics Syntax {#generics-syntax}

[![Rust by example - Generics][book~rust-by-example~generics~badge]][book~rust-by-example~generics]{{hi:Generics}}

Generics enable writing code that can work with different data types without having to write separate versions for each type. It is like creating a template, with placeholders that will be specified later.

Functions (`fn`), type aliases (`type`), structs (`struct`), enumerations (`enum`), unions, traits (`trait`), and implementations (`impl`) may be parameterized by _types_, _constants_, and _lifetimes_.

These parameters are listed in angle brackets (`<...>`), usually immediately after the name of the item and before its definition. For implementations, which don't have a name, they come directly after `impl`.

## Use Type Parameters {#type-parameters}

The following example demonstrates generic type parameters, the most common. Instead of specifying concrete types (like `i32`, `String`, or `bool`) when defining functions, structs, enums, methods, etc., you can use a placeholder, typically denoted by an uppercase letter like `T` (for "Type"). This placeholder acts as a stand-in for any type that the user of your code might provide.

You will also encounter type parameters named `K` for a key, `V` for a value, `F` for a function or closure, `S` for a state, `A` for an allocator, and `E` for an error, among others. These are just conventions and can be replaced with any valid identifier.

```rust,editable
{{#include ../../crates/language/examples/generics/generic_type_parameter.rs:example}}
```

See also the [[structs | Structs]], [[enums | Enums]], [[functions | Functions]], and [[traits | Traits]] sections for more details on how to use type parameters in those contexts.

## Use Lifetime Parameters {#lifetime-parameters}

"Lifetime generic parameters" (often just called "lifetime parameters" or "lifetimes") are another type of generics, similar to how you use `T` for a generic type, but instead of representing a type, they represent the _scope_ for which a _reference_ is valid.

- Lifetime parameters start with an apostrophe (') and are typically lowercase, like `'a`, `'b`, `'c`, etc.
- They are declared in angled brackets (`< >`) just like regular type parameters.
- Lifetime parameters must be listed first within `< >`, if mixed with type parameters or const generics.
- They are then used to annotate references.

Think of them as annotations that describe how the lifetime of one reference relates to another. They don't change how long a reference actually lives; they simply provide the compiler with the necessary information to check if your usage of references is safe.

When the Rust compiler can't definitively infer the relationships between the lifetimes of references, especially in functions or structs that hold references, you need to explicitly tell it using lifetime generic parameters.

Lifetime parameters are often used when a type (for example, a `struct`) contains references. They are used to establish relationships between the lifetimes of the references and the containing type.

See the [[lifetimes | Lifetimes]] chapter for more details.

The following provides examples of lifetime parameters:

```rust,editable
{{#include ../../crates/language/examples/generics/generic_lifetime.rs:example}}
```

## Const Generics {#const-generics}

"Const generics" allow you to define generic parameters that are _constant values_ rather than types or lifetimes.
This means you can parameterize types, traits, and functions with compile-time known values.

Const generic parameters are declared in the angle brackets alongside type and lifetime parameters:

- They start with the `const` keyword.
- They require a type annotation (e.g., `u32`). The allowed types are currently limited to integer types (`u8` to `u128`, `i8` to `i128`, `usize`, `isize`), `bool`, and `char`.
- You use them like regular constants within the item's definition.

Const generics allow you to:

- Create data structures that have their size or other properties determined at compile time by a constant,
- Implement traits for types based on constant values, i.e. define a single trait implementation that applies to all arrays (or your custom const-generic types) of a certain kind.
- Perform compile-time checks and optimizations. Const generics can shift certain checks from runtime to compile time. For example, you could define a function that only accepts a matrix if its dimensions meet certain criteria (e.g., for matrix multiplication), catching errors earlier in the development process.

The following example shows how to define a struct that holds an array of a specific length, which is determined at compile time by the constant generic parameter `N`. The struct can then be used with arrays of different lengths, as long as they match the specified constant:

```rust,editable
{{#include ../../crates/language/examples/generics/const_generics.rs:example}}
```

The following example demonstrates how to implement traits for types based on constant values:

```rust,editable
{{#include ../../crates/language/examples/generics/const_generics2.rs:example}}
```

## Related Topics {#related-topics}

- [[enums | Enums]].
- [[functions | Functions]].
- [[lifetimes | Lifetimes]].
- [[structs | Structs]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [[rust-patterns | Rust Patterns]].

</div>
