# Traits

```rust
{{#include ../../deps/tests/lang/traits.rs:example}}
```

{{i:Trait}} methods{{hi:Methods}} are in scope only when their trait is.

## Default implementation {#default-implementation}

```rust
{{#include ../../deps/tests/lang/traits2.rs:example}}
```

## Supertraits {#supertraits}

```rust
{{#include ../../deps/tests/lang/traits3.rs:example}}
```

## Newtype pattern {#newtype-pattern}

Unlike interfaces{{hi:Interfaces}} in languages like Java, C# or Scala, new traits{{hi:Traits}} can be implemented for _existing_ types.

```rust
{{#include ../../deps/tests/lang/traits4.rs:example}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern{{hi:Newtype pattern}}:

```rust
{{#include ../../deps/tests/lang/newtype.rs:example}}
```

## Traits as parameters {#traits-as-parameters}

```rust
{{#include ../../deps/tests/lang/traits_as_parameters.rs:example}}
```

## Multiple traits {#multiple-traits}

```rust
{{#include ../../deps/tests/lang/traits5.rs:example}}
```

## Return-position impl Trait {#return-position-impl-trait}

```rust
{{#include ../../deps/tests/lang/rpit.rs:example}}
```

## Generic traits {#generic-traits}

```rust
{{#include ../../deps/tests/lang/generic_traits.rs:example}}
```

## Associated types {#associated-types}

```rust
{{#include ../../deps/tests/lang/associated_types.rs:example}}
```

## Trait bounds {#trait-bounds}

```rust
{{#include ../../deps/tests/lang/trait_bounds.rs:example}}
```

## Constants in traits {#constants-in-traits}

```rust
{{#include ../../deps/tests/lang/const_in_traits.rs:example}}
```

## Async and traits {#async-and-traits}

See [Async][p-async]⮳

## See also

[Traits (blog)][blog-traits]⮳

[p-async]: ../categories/asynchronous/index.md
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: review
</div>
