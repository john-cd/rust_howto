# Traits

{{#include traits.incl.md}}

Traits are a way to define shared behavior that types (structs, enums, etc.) can implement. They are similar to interfaces or abstract classes in other languages.

## Trait Syntax {#skip}

```rust,editable
{{#include ../../crates/language/tests/traits/traits.rs:example}}
```

{{i:Trait}} methods{{hi:Methods}} are in scope only when their trait is.

## Default Implementation {#default-implementation}

Traits can provide default implementations for their methods. This allows types that implement the trait to use the default implementation or override it with their own custom implementation.

```rust,editable
{{#include ../../crates/language/tests/traits/traits2.rs:example}}
```

## Supertraits {#supertraits}

A trait can require that implementing types also implement other traits. These are called supertraits.

```rust,editable
{{#include ../../crates/language/tests/traits/traits3.rs:example}}
```

## "Newtype" Pattern {#newtype-pattern}

Unlike interfaces{{hi:Interfaces}} in languages like Java, C# or Scala, new traits{{hi:Traits}} can be implemented for _existing_ types.

```rust,editable
{{#include ../../crates/language/tests/traits/traits4.rs:example}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern{{hi:Newtype pattern}}.

The newtype pattern involves creating a new type (typically, a tuple struct with a single field) to wrap an existing type. This allows you to implement traits on the wrapper type, even if you don't own the original type or the trait.

```rust,editable
{{#include ../../crates/language/tests/traits/newtype.rs:example}}
```

## Traits as Parameters {#traits-as-parameters}

Traits can be used as parameters to functions, allowing the function to accept any type that implements the specified trait.

```rust,editable
{{#include ../../crates/language/tests/traits/traits_as_parameters.rs:example}}
```

## Multiple Traits {#multiple-traits}

```rust,editable
{{#include ../../crates/language/tests/traits/traits5.rs:example}}
```

## Return-position `impl` Trait {#return-position-impl-trait}

You can use `impl Trait` in the return type of a function to indicate that the function returns a type that implements a specific trait, without specifying the exact type.

This is useful when the exact type is complex or not relevant to the caller.

```rust,editable
{{#include ../../crates/language/tests/traits/rpit.rs:example}}
```

## Generic Traits {#generic-traits}

Traits can be generic, meaning they can have type parameters. This allows you to define traits that work with different types.

```rust,editable
{{#include ../../crates/language/tests/traits/generic_traits.rs:example}}
```

## Associated Types {#associated-types}

Traits can have associated types, which are types that are associated with the trait and can be used in its methods.

```rust,editable
{{#include ../../crates/language/tests/traits/associated_types.rs:example}}
```

## Trait Bounds {#trait-bounds}

```rust,editable
{{#include ../../crates/language/tests/traits/trait_bounds.rs:example}}
```

## Constants in Traits {#constants-in-traits}

Traits can define constants that implementing types can use.

```rust,editable
{{#include ../../crates/language/tests/traits/const_in_traits.rs:example}}
```

## Async and Traits {#async-and-traits}

See [Async][p-async]⮳.

## See Also {#skip}

[Traits (blog)][blog-traits]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[traits: review; async_traits: review new Rust features NOW](https://github.com/john-cd/rust_howto/issues/561)

- [What is the correct way to return an Iterator (or any other trait)?](https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait)

</div>
