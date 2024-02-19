# Parallel Tasks

## Mutate the elements of an array in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

The example uses the `rayon` crate, which is a data parallelism library for Rust.
`rayon` provides the [`par_iter_mut`][par_iter_mut] method for any parallel iterable data type. This is an iterator-like chain that potentially executes in parallel.

```rust,editable
{{#include ../../deps/examples/rayon-iter-mut.rs}}
```

## Test in parallel if any or all elements of a collection match a given predicate

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates using the [`rayon::any`][rayon::any] and [`rayon::all`][rayon::all] methods, which are parallelized counterparts to [`std::any`][std::any] and [`std::all`][std::all]. [`rayon::any`][rayon::any] checks in parallel whether any element of the iterator matches the predicate, and returns as soon as one is found. [`rayon::all`][rayon::all] checks in parallel whether all elements of the iterator match the predicate, and returns as soon as a non-matching element is found.

```rust,editable
{{#include ../../deps/examples/rayon-any-all.rs}}
```

## Search items using given predicate in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::find_any`][rayon::find_any] and [`par_iter`][par_iter] to search a vector in parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure argument of [`rayon::find_any`][rayon::find_any] `rayon` returns the first one found, not necessarily the first one.

Also note that the argument to the closure is a reference to a reference (`&&x`). See the discussion on [`std::find`][std::find] for additional details.

```rust,editable
{{#include ../../deps/examples/rayon-parallel-search.rs}}
```

## Sort a vector in parallel

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

This example will sort in parallel a vector of Strings.

Allocate a vector of empty Strings. `par_iter_mut().for_each` populates random values in parallel. Although [multiple options]
exist to sort an enumerable data type, [`par_sort_unstable`][par_sort_unstable] is usually faster than [stable sorting][stable sorting] algorithms.

```rust,editable
{{#include ../../deps/examples/rayon-parallel-sort.rs}}
```

## Map-reduce in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::filter`][rayon::filter] [`rayon::map`][rayon::map] and [`rayon::reduce`][rayon::reduce] to calculate the average age of `Person` objects whose age is over 30.

[`rayon::filter`][rayon::filter] returns elements from a collection that satisfy the given predicate. [`rayon::map`][rayon::map] performs an operation on every element, creating a new iteration, and [`rayon::reduce`][rayon::reduce] performs an operation given the previous reduction and the current element. Also shows use of [`rayon::sum`][rayon::sum] which has the same result as the reduce operation in this example.

```rust,editable
{{#include ../../deps/examples/rayon-map-reduce.rs}}
```

## Generate jpg thumbnails in parallel

[![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency] [![cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all .jpg files in the current directory then saves them in a new folder called `thumbnails`.

[`glob::glob_with`][glob::glob_with] finds jpeg files in current directory. `rayon` resizes images in parallel using [`par_iter`][par_iter] calling [`DynamicImage::resize`][DynamicImage::resize]

```rust,editable,no_run
{{#include ../../deps/examples/rayon-thumbnails.rs}}
```

{{#include ../refs/link-refs.md}}
