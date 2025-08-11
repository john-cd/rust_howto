# Iterators

{{#include iterators.incl.md}}

## Iterator Trait {#iterator-trait}

[![Rust by example - iterators][book~rust-by-example~iterators~badge]][book~rust-by-example~iterators]{{hi:Iterators}}

Iterators allow you to process a sequence of items. An iterator is any type that implements the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)↗{{hi:std::iter::Iterator}} trait. This trait requires only one required method: `next()`. Iterators are lazy, meaning they don't do any work until you ask for the next item (via `next()`).

```rust,noplayground
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... many other methods ...
}
```

The `next()` method returns an `Option<Self::Item>`. If there is a next item, it returns `Some(item)`, otherwise it returns `None`.

## Use Iterators {#use-iterators}

There are three common methods, which can create iterators from a collection:

- `iter()`, which iterates over `&T`,
- `iter_mut()`, which iterates over `&mut T`,
- `into_iter()`, which iterates over `T`.

Use `iter()` when you need to iterate over the elements of a collection without modifying them, `iter_mut()` when you need to modify them in place, and `into_iter()` when you want to consume the collection. The latter is often used when you're done with the original collection after iteration or when you want to transfer ownership of the elements. The following table summarizes the use cases:

| Feature | Mutability | Element Type | Ownership | Collection Use Afterwards |
|---|---|---|---|---|
| `iter()` | Immutable | `&T` | Borrows | Usable after |
| `iter_mut()` | Mutable | `&mut T` | Borrows | Usable after |
| `into_iter()` | Depends on type | `T` (typically) | Consumes | Not usable after |

```rust,editable
{{#include ../../crates/language/examples/iterators/iterators.rs:example}}
```

## Create Basic Iterators from Values and Closures {#create-an-iterator}

The [`std::iter`](https://doc.rust-lang.org/std/iter/index.html)↗{{hi:std::iter}} module in the standard library provide a number of functions to create basic iterators from values or closures:

```rust,editable
{{#include ../../crates/language/examples/iterators/simple_iterators.rs:example}}
```

## Use Iterator Adapters to Transform a Collection {#adapters}

Iterators are composable: it's very common to chain them together to do more complex forms of processing.

Functions which take an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)↗{{hi:std::iter::Iterator}} as input and return another `Iterator` are often called "iterator adapters". Common iterator adapters include `map`, `take`, and `filter`:

```rust,editable
{{#include ../../crates/language/examples/iterators/iterator_adapters.rs:example}}
```

## Return an Iterator from a Function or Method {#return-an-iterator}

The `impl SomeTrait` notation refers to an opaque type that implements a trait; it is only allowed in arguments and return types of functions and methods.

Iterator types produced by a chain of calls to iterator adapters (`map`, `take`, `filter`, etc.) can be quite complex and hard to write.
It is therefore common to use `impl Iterator<Item = T>` as the return type for functions or methods that return an iterator.

Advantageously, this also allows changing the implementation of the iterator within the function without breaking the calling code.

```rust,editable
{{#include ../../crates/language/examples/iterators/return_iterator.rs:example}}
```

## Accept Various Iterable Types as the Input of a Function or Method {#accept-various-iterable-types}

Many collection types in Rust implement [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)↗{{hi:std::iter::IntoIterator}}, which provides a method `into_iter()` that returns an iterator.

It is common to use `IntoIterator` as a trait bound for function parameters. This allows the input collection type to change.

```rust,editable
{{#include ../../crates/language/examples/iterators/consume_intoiterator.rs:example}}
```

## Make a Collection Iterable by Implementing `IntoIterator` {#implement-intoiterator}

You may define how a (collection) type is iterated by implementing the `IntoIterator` trait.

One benefit of implementing `IntoIterator` is that your type will work with the `for` loop syntax.

```rust,editable
{{#include ../../crates/language/examples/iterators/implement_intoiterator.rs:example}}
```

## References {#references}

- [Iterators][book~rust~iterators]{{hi:Iterators}}↗.
- [What is the correct way to return an Iterator (or any other trait)?][blog~stackoverflow~what-is-the-correct-way-to-return-an-iterator-or-any-other-trait]↗.
- [Implementing Iterator and IntoIterator in Rust](https://dev.to/wrongbyte/implementing-iterator-and-intoiterator-in-rust-3nio)↗.

## Related Topics {#related-topics}

- [[closures | Closures]].
- [[data-structures | Data Structures]].
- [[functional_programming | Functional Programming]].
- [[rust-patterns | Rust Patterns]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
