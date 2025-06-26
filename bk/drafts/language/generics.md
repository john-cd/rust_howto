# Generics

{{#include generics.incl.md}}

## Generics Syntax {#skip}

[![Rust by example - Generics][book~rust-by-example~generics~badge]][book~rust-by-example~generics]{{hi:Generics}}

Generics enable writing code that can work with different data types without having to write separate versions for each type. It is like creating a template, with placeholders that will be specified later.

Functions, type aliases (`type`), structs (`struct`), enumerations (`enum`), unions, traits (`trait`), and implementations (`impl`) may be parameterized by types, constants, and lifetimes. These parameters are listed in angle brackets (`<...>`), usually immediately after the name of the item and before its definition.

## Type Parameters {#type-parameters}

The following example demonstrates type parameters, the most common:

```rust,editable
{{#include ../../crates/language/examples/generics/generics.rs:example}}
```

## Lifetime Parameters {#lifetime-parameters}

Lifetime parameters are used when a `struct` contains references. They are used to establish relationships between the lifetimes of the references in the `struct`. See [[lifetimes | Lifetimes]] for more details.

```rust,editable
{{#include ../../crates/language/examples/generics/generic_lifetime.rs:example}}
```

## Const Generics {#const-generics}

"const generics" allow you to define generic parameters that are constant values rather than types. This is useful for working with arrays of fixed sizes (to parameterize their length):

```rust,editable
{{#include ../../crates/language/examples/generics/const_generics.rs:example}}
```

## Generics by Type {#skip}

### Generic Structs {#generic-structs-stub}

Generic structs are declared by adding one or more type parameters between `<` and `>`, after the name of the struct. See [[structs | Structs]] for more details.

### Generic Enums {#generic-enums-stub}

Generic enums are declared similarly to generic structs, with type parameters between `<` and `>` after the enum name.
The most common example is the `Result` enum.{{hi:Result}} It can either hold a value of type `T` (in the `Ok` variant) or an error value of type `E` (in the `Err` variant):

```rust,noplayground
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

See [[enums | Enums]] for more details.

### Generic Functions {#generic-functions-stub}

Generic functions allow you to write code that can operate on parameters of various types without needing to rewrite the function for each specific type.
See [[functions | Functions]] for more details.

### Generic Traits {#generic-traits-stub}

See [[traits | Traits]] for more details.

## See Also {#skip}

- [[enums | Enums]].
- [[functions | Functions]].
- [[lifetimes | Lifetimes]].
- [[rust-patterns | Rust Patterns]].
- [[structs | Structs]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO make table?
</div>
