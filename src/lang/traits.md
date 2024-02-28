# Traits

```rust,editable
{{#include ../../deps/tests/traits.rs}}
```

Trait {{i:methods}} are in scope only when their trait is.

## Default implementation

```rust,editable
{{#include ../../deps/tests/traits2.rs}}
```

## Supertraits

```rust,editable
{{#include ../../deps/tests/traits3.rs}}
```

## Newtype pattern

Unlike {{i:interfaces}} in languages like Java, C# or Scala, new {{i:traits}} can be implemented for _existing_ types.

```rust,editable
{{#include ../../deps/tests/traits4.rs}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the {{i:newtype pattern}}:

```rust,editable
{{#include ../../deps/tests/newtype.rs}}
```

## Traits as parameters

```rust,editable
{{#include ../../deps/tests/traits_as_parameters.rs}}
```

## Multiple traits

```rust,editable
{{#include ../../deps/tests/traits5.rs}}
```

## Return-position impl Trait

```rust,editable
{{#include ../../deps/tests/rpit.rs}}
```

## Generic traits

```rust,editable
{{#include ../../deps/tests/generic_traits.rs}}
```

## Associated types

```rust,editable
{{#include ../../deps/tests/associated_types.rs}}
```

## Trait bounds

```rust,editable
{{#include ../../deps/tests/trait_bounds.rs}}
```

## Constants in traits

```rust,editable
{{#include ../../deps/tests/const_in_traits.rs}}
```

## Async and traits

See [Async](../categories/asynchronous/index.md)

## See also

[Traits (blog)][blog-traits]⮳

{{#include ../refs/link-refs.md}}
