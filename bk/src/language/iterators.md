# Iterators

{{#include iterators.incl.md}}

## Iterator Trait {#skip}

[![Rust by example - iterators][book-rust-by-example-iterators-badge]][book-rust-by-example-iterators]{{hi:Iterators}}

Iterators allow you to process a sequence of items. An iterator is any type that implements the `Iterator` trait. This trait requires only one method: `next()`. Iterators are lazy, meaning they don't do any work until you ask for the next item (via `next()`).

```rust,noplayground
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... other methods ...
}
```

The `next()` method returns an `Option<Self::Item>`. If there is a next item, it returns `Some(item)`, otherwise it returns `None`.

## Create an Iterator {#create-an-iterator}

Many types in Rust implement `IntoIterator`, which provides a method `into_iter()` that returns an iterator.

```rust,editable
{{#include ../../crates/language/tests/iterators/iterators.rs:example}}
```

## Related Topics {#skip}

- [[closures | Closures]].
- [[data-structures | Data Structures]].
- [[functional_programming | Functional Programming]].
- [[vectors | Vectors]].

## See Also

[Iterators][book-rust-iterators]{{hi:iterators}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[iterators: edit NOW](https://github.com/john-cd/rust_howto/issues/546)
</div>
