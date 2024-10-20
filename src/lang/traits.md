# Traits

```rust
{{#include ../../deps/tests/traits.rs}}
```

Trait methods{{hi:Methods}} are in scope only when their trait is.

## Default implementation

```rust
{{#include ../../deps/tests/traits2.rs}}
```

## Supertraits

```rust
{{#include ../../deps/tests/traits3.rs}}
```

## Newtype pattern

Unlike interfaces{{hi:Interfaces}} in languages like Java, C# or Scala, new traits{{hi:Traits}} can be implemented for _existing_ types.

```rust
{{#include ../../deps/tests/traits4.rs}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern{{hi:Newtype pattern}}:

```rust
{{#include ../../deps/tests/newtype.rs}}
```

## Traits as parameters

```rust
{{#include ../../deps/tests/traits_as_parameters.rs}}
```

## Multiple traits

```rust
{{#include ../../deps/tests/traits5.rs}}
```

## Return-position impl Trait

```rust
{{#include ../../deps/tests/rpit.rs}}
```

## Generic traits

```rust
{{#include ../../deps/tests/generic_traits.rs}}
```

## Associated types

```rust
{{#include ../../deps/tests/associated_types.rs}}
```

## Trait bounds

```rust
{{#include ../../deps/tests/trait_bounds.rs}}
```

## Constants in traits

```rust
{{#include ../../deps/tests/const_in_traits.rs}}
```

## Async and traits

See [Async][p-async]⮳

## See also

[Traits (blog)][blog-traits]⮳

[p-async]: ../categories/asynchronous/index.md
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: review
</div>
