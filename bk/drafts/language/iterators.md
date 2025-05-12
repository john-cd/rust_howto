# Iterators

{{#include iterators.incl.md}}

## Iterator Trait {#skip}

[![Rust by example - iterators][book-rust-by-example-iterators-badge]][book-rust-by-example-iterators]{{hi:Iterators}}

Iterators allow you to process a sequence of items. An iterator is any type that implements the `Iterator` trait. This trait requires only one method: `next()`. Iterators are lazy, meaning they don't do any work until you ask for the next item (via `next()`).

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
{{#include ../../crates/language/tests/iterators/iterators.rs:example}}
```

## Create Basic Iterators from Values and Closures {#create-an-iterator}

The `std::iter` module in the standard library provide a number of functions to create basic iterators from values or closures:

```rust,editable
{{#include ../../crates/language/tests/iterators/simple_iterators.rs:example}}
```

## Use Iterator Adapters to Transform a Collection {#adapters}

Iterators are composable: it's very common to chain them together to do more complex forms of processing.

Functions which take an `Iterator` and return another `Iterator` are often called "iterator adapters". Common iterator adapters include `map`, `take`, and `filter`.

```rust,editable
{{#include ../../crates/language/tests/iterators/iterator_adapters.rs:example}}
```

## Accept Various Iterable Types as the Input of a Function or Method {#accept-various-iterable-types}

Many types in Rust implement `IntoIterator`, which provides a method `into_iter()` that returns an iterator.

It is common to use `IntoIterator` as a trait bound for function parameters. This allows the input collection type to change, so long as it is still an iterator. Additional bounds can be specified by restricting on Item.

```rust,editable
{{#include ../../crates/language/tests/iterators/consume_intoiterator.rs:example}}
```

## Implement an Iterator {#implement-an-iterator}

By implementing `IntoIterator` for a type, you define how it will be converted to an iterator. It is common for types which describe a collection of some kind.

One benefit of implementing `IntoIterator` is that your type will work with Rust's `for` loop syntax.

```rust,editable
{{#include ../../crates/language/tests/iterators/implement_intoiterator.rs:example}}
```

## Related Topics {#skip}

- [[closures | Closures]].
- [[data-structures | Data Structures]].
- [[functional_programming | Functional Programming]].
- [[vectors | Vectors]].

## References {#skip}

- [Iterators][book-rust-iterators]{{hi:iterators}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[iterators: review; return impl Iterator](https://github.com/john-cd/rust_howto/issues/546)

- [What is the correct way to return an Iterator (or any other trait)?](https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait)

</div>
