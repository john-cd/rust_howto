# Slices

{{#include slices.incl.md}}{{hi:Slices}}

## Slices {#skip}

[![Rust by example - slices][book-rust-by-example-slices-badge]][book-rust-by-example-slices]

A slice is a reference to a contiguous sequence of elements within a collection. This means it provides a view into a portion of an array, vector, or string, without owning the underlying data.

- Slices are references, so they don't own the data they point to. This makes them efficient, as they avoid unnecessary copying.
- They allow you to work with parts of a collection, enabling flexibility in data manipulation.
- Slices are often created using the `&[T]` syntax, where `T` is the type of the elements in the slice. For string slices, the type is `&str`.

### Common Use Cases {#skip}

- Accessing Subsets of Data: Slices are frequently used to access specific portions of arrays, vectors, or strings without copying the entire collection.
- Function Arguments: Slices are often used as function arguments when you want to operate on a part of a collection.
- String Manipulation: string slices (`&str`) are a common way to reference a string literal or a portion of a `String`.

## Create Slices {#slices}

Slices can be created by referencing a portion of a collection using a range. The range can be specified using `[start..end]`, where `start` is the index of the first element to include, and `end` is the index of the element after the last one to include. If `start` is omitted, the slice starts from the beginning of the collection. If `end` is omitted, the slice extends to the end of the collection.

```rust,editable
{{#include ../../crates/language/tests/slices/slices.rs:example}}
```

## Useful Functions {#skip}

- [`slice::sort_by`][primitive-slice::sort_by]⮳.
- [`slice::split_at`][primitive-slice::split_at]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[review NOW](https://github.com/john-cd/rust_howto/issues/1347)
</div>
