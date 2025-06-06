# Traits

{{#include traits.incl.md}}

## Trait Syntax {#trait}

Traits are a way to define shared behavior that types (structs, enums, etc.) can implement. They are similar to interfaces or abstract classes in other languages. {{i:Trait}} methods{{hi:Methods}} are in scope only when their trait is.

```rust,editable
{{#include ../../crates/language/tests/traits/traits.rs:example}}
```

## Default Method Implementation {#default-implementation}

Traits can provide default implementations for their methods. This allows types that implement the trait to use the default implementation or override it with their own custom implementation:

```rust,editable
{{#include ../../crates/language/tests/traits/trait_default_implementation.rs:example}}
```

## Supertraits {#supertraits}

A trait can require that implementing types also implement other traits. These are called supertraits:

```rust,editable
{{#include ../../crates/language/tests/traits/supertraits.rs:example}}
```

## Implement a Local Trait for a Type Defined Elsewhere {#traits-types-elsewhere}

You can implement a trait defined in your crate {{hi:Traits}} on types defined outside of it - simply by writing a `impl` block.
This is often used to extend the functionality of an external type:

```rust,editable
{{#include ../../crates/language/tests/traits/extend_external_type.rs:example}}
```

## Implement a Trait on a Type, Both Defined Elsewhere, with the "Newtype" Pattern {#newtype-pattern}

One restriction to note is that you can implement a trait on a type only if at least one of the trait or the type is _local to your crate_. If neither are, use the "newtype" pattern{{hi:Newtype pattern}}.

The newtype pattern involves creating a new local type (typically, a tuple struct with a single field) to wrap an existing type. This allows you to implement traits on the wrapper type, even if you don't own the original type or the trait.

```rust,editable
{{#include ../../crates/language/tests/traits/newtype.rs:example}}
```

## Trait Bounds {#trait-bounds}

Trait bounds TODO

```rust,editable
{{#include ../../crates/language/tests/traits/trait_bounds.rs:example}}
```

Traits can be used as bounds to generic parameters of a functions, allowing the function to accept any type that implements the specified trait.

```rust,editable
{{#include ../../crates/language/tests/traits/traits_as_parameters.rs:example}}
```

### Multiple Traits {#multiple-traits}

Trait bounds with multiple traits TODO

```rust,editable
{{#include ../../crates/language/tests/traits/multiple_traits.rs:example}}
```

## Simplify Method Signatures with `impl Trait` {#impl-trait}

`impl Trait` specifies an unnamed but concrete type that implements a specific trait. It can only appear in argument position (where it can act as an anonymous type parameter to functions) and in return position (where it can act as an opaque return type).

`impl Trait` is essentially syntactic sugar for a generic type parameter like `<T: Trait>`, except that, in argument position, the type is anonymous and doesn't appear in the generic parameter list of a function. In return position, unlike with a generic type parameter, the function, not the caller, chooses the return type.

Do not confuse `impl Trait` with `dyn Trait`. The [[trait_objects | Trait Objects]] chapter explains the difference.

```rust,editable
TODO
```

## Return-position `impl` Trait {#return-position-impl-trait}

You can use `impl Trait` in the return type of a function to indicate that the function returns a type that implements a specific trait, without specifying the exact type.

This is useful when the exact type is complex or not relevant to the caller.

```rust,editable
{{#include ../../crates/language/tests/traits/rpit.rs:example}}
```

## Generic Traits {#write-generic-traits}

Traits can be generic, meaning they can have type parameters. This allows you to define traits that work with different types.

```rust,editable
{{#include ../../crates/language/tests/traits/generic_traits.rs:example}}
```

## Associated Types in Traits {#associated-types}

Traits can have associated types, which are types that are associated with the trait and can be used in its methods.

```rust,editable
{{#include ../../crates/language/tests/traits/associated_types.rs:example}}
```

A common pattern is a generic type (with a default) and an associated type.
The use of an associated type eliminates the need to write generic type parameters in many places.

```rust,noplayground
/// A trait that represents the ability to add two values together.
trait Add<Rhs = Self> {
    /// The type of the result of the addition.
    type Output;

    /// Adds two values together.
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

## Associated Constants in Traits {#constants-in-traits}

Traits can also define constants that implementing types can use.

```rust,editable
{{#include ../../crates/language/tests/traits/const_in_traits.rs:example}}
```

## Sealed Trait Pattern {#sealed-trait-pattern}

```rust,editable
{{#include ../../crates/language/tests/traits/sealed_trait_pattern.rs:example}}
```

## Blanket Implementations {#blanket-implementations}

```rust,editable
{{#include ../../crates/language/tests/traits/blanket_implementations.rs:example}}
```

Blanket `impl` apply globally and can lead to conflicts if overused.

## Async and Traits {#async-and-traits}

This topic is covered in the [Async][p-async]⮳ chapter.

## Related Topics {#skip}

- [[enums | Enums]].
- [[generics | Generics]].
- [[structs | Structs]].
- [[trait_objects | Trait Objects]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [Traits (blog)][blog-traits]⮳.
- [What is the correct way to return an Iterator (or any other trait)?](https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait)⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
