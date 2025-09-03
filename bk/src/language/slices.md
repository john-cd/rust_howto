# Slices

{{#include slices.incl.md}}{{hi:Slices}}

## Slice Basics {#slices}

[![Rust by example - slices][book~rust-by-example~slices~badge]][book~rust-by-example~slices]

A slice is a view to a contiguous sequence of elements within a collection (array, vector, etc.).

The slice type is `[T]`, where `T` represents the element type (e.g. `u8`). Because it is dynamically sized, it is most often found behind a pointer:

- Immutable slice reference: `&[T]`,
- Mutable slice reference: `&mut [T]`,
- Boxed slice (on the heap): `Box<[T]>`,
- Shareable reference-counted slice: `Rc<[T]>` or `Arc<[T]>`.

A few important points:

- Slices _do not own_ the data they point to. This makes them efficient, as they avoid unnecessary copying.
- They allow you to work with parts of a collection, enabling flexibility in data manipulation.
- Slice references are "fat pointers", i.e. they store a pointer and the length of the sequence they refer to, thus they have twice the size of pointers to regular `Sized` types.
- As a primitive type, `slice` implements a large number of methods, for example to `swap` two elements; `reverse` the order of elements in the slice in place; `iter`ate elements; create `chunks` or overlapping windows into the slice; figure out if the slice `contains` an element with the given value, etc. See also:
  - [`slice::sort_by`][primitive~slice::sort_by]↗,
  - [`slice::split_at`][primitive~slice::split_at]↗.

Empty slices can be created:

```rust,editable
let empty = &x[..0]; // Same as `&x[0..0]`.
assert_eq!(empty, &[]);
```

### Common Use Cases {#common-use-cases}

- Accessing Subsets of Data: Slices are frequently used to access specific portions of arrays, vectors, or strings without copying the entire collection.
- Function Arguments: Slices are often used as function arguments when you want to operate on a part of a collection.
- String Manipulation: string slices (`&str`) are a common way to reference a string literal or a portion of a `String` (see below).

## Create Slices from Arrays or Vectors {#create-slices}

Slices can be created by referencing a portion of a collection using a range. The range can be specified using `[start..end]`, where `start` is the index of the first element to include, and `end` is the index of the element after the last one to include. If `start` is omitted, the slice starts from the beginning of the collection. If `end` is omitted, the slice extends to the end of the collection. `[..]` refers to the entire collection.

```rust,editable
{{#include ../../crates/language/examples/slices/slices.rs:example}}
```

## Use a Slice as a Function Argument {#use-slice-as-function-argument}

You will commonly see slices as function arguments, because `Vec<T>` implements `Deref<Target = [T]>`. Therefore, you can simply pass a `&Vec<T>` to a function that accepts `&[T]`:

```rust,editable
{{#include ../../crates/language/examples/slices/slice_as_argument.rs:example}}
```

The example also shows that immutable (and mutable) slices implement `IntoIterator`, yielding references to each slice element.

## String Slices {#string-slices}

String slices, denoted `&str`, are very common. In particular, string literals are of type `&'static str`. String slices are essentially regular slices, with the additional requirement that they must always be valid UTF-8. Owned strings (`String`) dereference to `&str`, just like `Vec<T>` dereferences to `&[T]`.

```rust,editable
{{#include ../../crates/language/examples/slices/string_slices.rs:example}}
```

See the [[string | String]] chapter for more details.

## Related Topics {#related-topics .skip}

- [[strings | Strings]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
