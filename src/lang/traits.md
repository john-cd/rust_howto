# Traits

```rust,editable
{{#include ../../deps/examples/traits.rs}}
```

Trait methods are in scope only when their trait is.

## Default implementation

```rust,editable
{{#include ../../deps/examples/traits2.rs}}
```

## Supertraits

```rust,editable
{{#include ../../deps/examples/traits3.rs}}
```

## Newtype pattern

Unlike interfaces in languages like Java, C# or Scala, new traits can be implemented for _existing_ types.

```rust,editable
{{#include ../../deps/examples/traits4.rs}}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the newtype pattern:

```rust,editable
{{#include ../../deps/examples/newtype.rs}}
```

## Traits as parameters

```rust,editable
{{#include ../../deps/examples/traits_as_parameters.rs}}
```

## Multiple traits

```rust,editable
{{#include ../../deps/examples/traits5.rs}}
```

## Return-position impl Trait

```rust,editable
{{#include ../../deps/examples/rpit.rs}}
```

## Generic traits

```rust,editable
{{#include ../../deps/examples/generic_traits.rs}}
```

## Associated types

```rust,editable
{{#include ../../deps/examples/associated_types.rs}}
```

## Trait bounds

```rust,editable
{{#include ../../deps/examples/trait_bounds.rs}}
```

## Constants in traits

```rust,editable
{{#include ../../deps/examples/const_in_traits.rs}}
```

## Async and traits

See [Async](../concurrency/async.md)

## See also

[Traits (blog)]( https://blog.rust-lang.org/2015/05/11/traits.html )
