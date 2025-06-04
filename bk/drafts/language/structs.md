# Structs

{{#include structs.incl.md}}

## Struct Syntax {#struct}

Structs are similar to tuples, but each piece of data in a struct has a name. Because of these names, it’s clearer what the data means. Adding names to the elements of a tuple would require you to remember the index rather than the name.


[book-rust-by-example-structs-badge]: https://img.shields.io/badge/Rust%20by%20Example-Structs-blue
[book-rust-by-example-structs]: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
[book-rust-reference-visibility-and-privacy]: https://doc.rust-lang.org/reference/visibility-and-privacy.html

[![Rust by example - Structs][book-rust-by-example-structs-badge]][book-rust-by-example-structs]{{hi:Structs}}

Structs are custom data types that allow you to group related data together.

The following example demonstrates how to define a `struct` and create a instance of it:

```rust,editable
{{#include ../../crates/language/tests/structs/structs.rs:example}}
```

Struct fields{{hi:Fields}} follow the general rule of everything being private by default,{{hi:Private by default}} unless annotated with [`pub`][book-rust-reference-visibility-and-privacy]{{hi:Visibility}}⮳.

### Tuple Structs and Unit-like Structs {#tuple-structs}

You can also define tuple struct (no field names) and "Unit-like" structs (without fields).

Unit-like structs are so named, because they behave similarly to `()`.

```rust,editable
{{#include ../../crates/language/tests/structs/tuple_structs.rs:example}}
```

## Methods and Associated Functions {#methods}

Each struct is allowed to have multiple `impl` blocks.

We’ll see a case in which multiple `impl` blocks are useful in Chapter 10, where we discuss generic types and traits.

Methods are functions that are associated with a specific instance of a struct. They are defined within an impl (implementation) block and their first parameter is always self, &self, or &mut self.

Associated functions are also defined within an impl block but do not take self as a parameter. They are often used as constructors (like String::from).


Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

```rust,noplayground
p1.distance(&p2);
(&p1).distance(&p2);
```

```rust,editable
{{#include ../../crates/language/tests/structs/structs_impl.rs:example}}
```

## Implement a Trait for a Struct {#implement-trait-for-struct}

a trait defines a set of methods that a type can implement. When a struct implements a trait, it promises to provide concrete implementations for the methods defined by that trait. This is how Rust achieves polymorphism (often called "duck typing" in other languages, though Rust's trait system is statically checked).

impl TraitName for StructName { ... }:

impl: The keyword to start an implementation block.
TraitName: The name of the trait you want to implement.
for StructName: The specific struct for which you are providing the implementation.
Method Signatures Must Match:

The methods you define within the `impl` block must match the signatures (name, parameters, return type) defined in the trait.


self, &self, &mut self:

Like regular struct methods, trait methods also use self, &self, or &mut self as their first parameter to specify how they interact with the instance of the struct.
&self: Borrows the struct instance immutably. Used for methods that only read data.
&mut self: Borrows the struct instance mutably. Used for methods that need to modify the struct's data.
self: Takes ownership of the struct instance. This is less common for typical trait methods unless the method is intended to consume the instance.

## Struct Initialization {#struct-init}

It is common to define a function or an associated function to initialize the struct:

```rust,editable
{{#include ../../crates/language/tests/structs/structs_init.rs:example}}
```

## Create Instances from Other Instances with Struct Update Syntax {#update-struct}

You may update a `struct` by using the `..previous_instance` to fill in the rest.

```rust,editable
{{#include ../../crates/language/tests/structs/structs_update.rs:example}}
```

## Design Structs for Ownership

We want each instance of the User struct to own all of its data. This is generally preferred because it means the data within the struct is valid for as long as the struct itself is valid.

## Implementing Common Standard Library Traits

In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
https://zerotomastery.io/blog/rust-struct-guide/
</div>
