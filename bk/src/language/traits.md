# Traits

{{#include traits.incl.md}}

## Trait Syntax {#trait}

Traits are a way to define shared behavior that types can implement. They are similar to interfaces or abstract classes in other languages.

A trait consists of _associated items_: functions (including methods), types, and constants. {{i:Trait}} items are in scope only when their trait is.

The following example demonstrates a trait with one method and its implementation on a type (here, a `struct`). Trait functions and methods typically omit the function body by replacing it with a semicolon. The trait implementation then must provide the function body:

```rust,editable
{{#include ../../crates/language/examples/traits/traits.rs:example}}
```

A trait can be implemented by a struct, an enum, but also by a union, a primitive type, a sequence (slice or array), a tuple, a function pointer or a reference / pointer type:

```rust,editable
{{#include ../../crates/language/examples/traits/trait_types.rs:example}}
```

### Provide a Default Implementation for a Trait's Function or Method {#default-implementation}

Traits can provide default implementations for their functions or methods. This allows types that implement the trait to either use the default implementation or override it with their own custom implementation:

```rust,editable
{{#include ../../crates/language/examples/traits/trait_default_implementation.rs:example}}
```

### Define Associated Types in Traits {#associated-types}

Traits can have associated types, which are types that can be used in its functions and methods:

```rust,editable
{{#include ../../crates/language/examples/traits/associated_types.rs:example}}
```

An associated type declaration can include generic parameters and trait bounds (see [[generics | Generics]] for more details):

```rust,noplayground
use std::fmt::Debug;
use std::fmt::Display;

trait Tr {
    type Item;
    // Associated type with a trait bound:
    type Item2: Display;
    // Associated type with multiple trait bounds:
    type Item3: Debug + Display;
    // Associated type with a generic type parameter:
    type Item4<T>;
    // Associated type with a lifetime, a generic type parameter, and a const generic:
    type Item5<'a, T, const N: usize>;
    // Associated type with a `where` bound:
    type Item6<T>
    where
        T: Clone;
}
```

A common pattern is a generic trait, with a generic type parameter with a default, plus an associated type for the output.
The use of an associated type eliminates the need to write generic type parameters in many places:

```rust,noplayground
/// A trait that represents the ability to add two values together.
/// Similar to `core::ops::Add`.
trait Add<Rhs = Self> {
    /// The type of the result of the addition.
    type Output;

    /// Adds two values together.
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

### Define Associated Constants in Traits {#constants-in-traits}

Traits can also define constants that implementing types can use. Associated constants may omit the equals sign and expression to indicate implementations must define the constant value:

```rust,editable
{{#include ../../crates/language/examples/traits/const_in_traits.rs:example}}
```

## Require that a Type Implement Other Traits (with Supertraits) {#supertraits}

A trait can require that implementing types also implement other traits. These are called supertraits:

```rust,editable
{{#include ../../crates/language/examples/traits/supertraits.rs:example}}
```

## Implement a Local Trait for a Type Defined Elsewhere {#traits-types-elsewhere}

You can implement a trait defined in your crate {{hi:Traits}} on types defined outside of it - simply by writing a `impl` block.
This is often used to extend the functionality of an external (e.g., standard library) type:

```rust,editable
{{#include ../../crates/language/examples/traits/extend_external_type.rs:example}}
```

## Implement a Trait on a Type, Both Defined Elsewhere, with the "Newtype" Pattern {#newtype-pattern}

One restriction to note is that you can implement a trait on a type only if at least one of the trait or the type is _local to your crate_. If neither are, use the "newtype" pattern{{hi:Newtype pattern}}.

The newtype pattern involves creating a new local type (typically, a tuple struct with a single field) to wrap an existing type. This allows you to implement traits on the wrapper type, even if you don't own the original type or the trait.

```rust,editable
{{#include ../../crates/language/examples/traits/newtype.rs:example}}
```

## Prohibit Trait Implementation with the Sealed Trait Pattern {#sealed-trait-pattern}

The sealed trait pattern is a way to prevent crates from implementing a trait that they don't own. This allows the author of the trait to add new items to it in the future without that being a breaking change for downstream users.

This is typically achieved by defining a public trait that has a dependency on another "sealer" trait which is kept private to the crate. Because the sealer trait is not accessible outside of the crate, no downstream crate can satisfy the necessary trait bounds to implement the public-facing trait. More recently, the `#[sealed]` attribute has been introduced as a more direct and cleaner way to apply this pattern:

```rust,editable
{{#include ../../crates/language/examples/traits/sealed_trait_pattern.rs:example}}
```

## Provide a Blanket Implementation of a Trait {#blanket-implementations}

A "blanket implementation" implements a trait for any type that satisfies a given set of trait bounds. This allows for broad, generic functionality to be applied across a wide range of different types automatically, promoting code reuse and extensibility. A well-known example in Rust's standard library is the implementation of the `ToString` trait for any type that implements the `Display` trait, allowing any displayable type to be converted into a string.

Beware that blanket `impl` apply globally and can lead to conflicts if overused.

```rust,editable
{{#include ../../crates/language/examples/traits/blanket_implementations.rs:example}}
```

## Define Generic Traits that Work with Multiple Types {#generic-traits}

Generic traits allow you to define a shared set of behaviors that can be implemented by multiple types, enabling code reuse and polymorphism:

```rust,editable
{{#include ../../crates/language/examples/traits/generic_traits.rs:example}}
```

## Use a Trait Bound to Guarantee that a Generic Type Implements a Trait {#trait-bounds}

Trait bounds are often used with generics to specify that a generic type must implement a particular trait. Traits bounds can be applied to generic parameters of a function, allowing the function to accept any type that implements the specified trait, or used in a type definition or `impl` block:

```rust,editable
{{#include ../../crates/language/examples/traits/trait_bounds.rs:example}}
```

Trait bounds are specified using the `TypeParameter: Trait` syntax, where `TypeParameter` is a generic type parameter and `Trait` is the trait that the type must implement. There are written within `< >` or in a separate `where` clause, which can be more readable. The `impl Trait` syntax may also used:

```rust,editable
{{#include ../../crates/language/examples/traits/trait_bounds2.rs:example}}
```

Bounds with multiple traits are possible:

```rust,editable
{{#include ../../crates/language/examples/traits/trait_bounds_multiple_traits.rs:example}}
```

For more details on trait bounds, see the [[generics | Generics]] and [[impl_trait | Impl Trait]] chapters.

## Use Async with Traits {#async-and-traits}

This topic is covered in the [Async][p~async]⮳ chapter.

## Related Topics {#skip}

- [[enums | Enums]].
- [[generics | Generics]].
- [[impl_trait | impl Trait]].
- [[structs | Structs]].
- [[trait_objects | Trait Objects]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [Traits (blog)][blog~traits]⮳.
- [What is the correct way to return an Iterator (or any other trait)?](https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait)⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
