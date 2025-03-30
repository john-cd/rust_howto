# Slices

{{#include slices.incl.md}}{{hi:Slices}}

A slice is a reference to a contiguous sequence of elements within a collection. This means it provides a view into a portion of an array, vector, or string, without owning the underlying data.

- Slices are references, so they don't own the data they point to. This makes them efficient, as they avoid unnecessary copying.
- They allow you to work with parts of a collection, enabling flexibility in data manipulation.
- Slices are often created using the `&[T]` syntax, where `T` is the type of the elements in the slice. For string slices, the type is `&str`.

## Common Use Cases

- Accessing Subsets of Data: Slices are frequently used to access specific portions of arrays, vectors, or strings without copying the entire collection.
- Function Arguments: Slices are often used as function arguments when you want to operate on a part of a collection.
- String Manipulation: string slices (`&str`) are a common way to reference a string literal or a portion of a `String`.

## Create Slices {#slices}

```rust,editable
{{#include ../../crates/language/tests/feat/slices.rs:example}}
```

## Useful Functions {#skip}

- [`slice::sort_by`][primitive-slice::sort_by]⮳.
- [`slice::split_at`][primitive-slice::split_at]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[slices: add text](https://github.com/john-cd/rust_howto/issues/558)
</div>
