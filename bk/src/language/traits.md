# Traits

{{#include traits.incl.md}}

```rust,editable
{{#include ../../crates/language/tests/traits/traits.rs:example}}
```

{{i:Trait}} methods{{hi:Methods}} are in scope only when their trait is.

## Default Implementation {#default-implementation}

```rust,editable
{{#include ../../crates/language/tests/traits/traits2.rs:example}}
```

## Supertraits {#supertraits}

```rust,editable
{{#include ../../crates/language/tests/traits/traits3.rs:example}}
```

## "Newtype" Pattern {#newtype-pattern}

Unlike interfaces{{hi:Interfaces}} in languages like Java, C# or Scala, new traits{{hi:Traits}} can be implemented for _existing_ types.

```rust,editable
{{#include ../../crates/language/tests/traits/traits4.rs:example}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern{{hi:Newtype pattern}}:

```rust,editable
{{#include ../../crates/language/tests/traits/newtype.rs:example}}
```

## Traits as Parameters {#traits-as-parameters}

```rust,editable
{{#include ../../crates/language/tests/traits/traits_as_parameters.rs:example}}
```

## Multiple Traits {#multiple-traits}

```rust,editable
{{#include ../../crates/language/tests/traits/traits5.rs:example}}
```

## Return-position `impl` Trait {#return-position-impl-trait}

`impl`

```rust,editable
{{#include ../../crates/language/tests/traits/rpit.rs:example}}
```

## Generic Traits {#generic-traits}

```rust,editable
{{#include ../../crates/language/tests/traits/generic_traits.rs:example}}
```

## Associated Types {#associated-types}

```rust,editable
{{#include ../../crates/language/tests/traits/associated_types.rs:example}}
```

## Trait Bounds {#trait-bounds}

```rust,editable
{{#include ../../crates/language/tests/traits/trait_bounds.rs:example}}
```

## Constants in Traits {#constants-in-traits}

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
</div>
