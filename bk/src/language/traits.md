# Traits {#traits}

{{#include traits.incl.md}}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits.rs:example}}
```

{{i:Trait}} methods{{hi:Methods}} are in scope only when their trait is.

## Default implementation {#default-implementation}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits2.rs:example}}
```

## Supertraits {#supertraits}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits3.rs:example}}
```

## "Newtype" pattern {#newtype-pattern}

Unlike interfaces{{hi:Interfaces}} in languages like Java, C# or Scala, new traits{{hi:Traits}} can be implemented for _existing_ types.

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits4.rs:example}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern{{hi:Newtype pattern}}:

```rust,editable
{{#include ../../crates/ex/language/tests/feat/newtype.rs:example}}
```

## Traits as parameters {#traits-as-parameters}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits_as_parameters.rs:example}}
```

## Multiple traits {#multiple-traits}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/traits5.rs:example}}
```

## Return-position `impl` Trait {#return-position-impl-trait}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/rpit.rs:example}}
```

## Generic traits {#generic-traits}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/generic_traits.rs:example}}
```

## Associated types {#associated-types}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/associated_types.rs:example}}
```

## Trait bounds {#trait-bounds}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/trait_bounds.rs:example}}
```

## Constants in traits {#constants-in-traits}

```rust,editable
{{#include ../../crates/ex/language/tests/feat/const_in_traits.rs:example}}
```

## Async and traits {#async-and-traits}

See [Async][p-async]⮳

## See also

[Traits (blog)][blog-traits]⮳

[p-async]: ../categories/asynchronous/index.md
{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[traits: review (P1)](https://github.com/john-cd/rust_howto/issues/561)
[async_traits: review new Rust features (P1)](https://github.com/john-cd/rust_howto/issues/216)

</div>
