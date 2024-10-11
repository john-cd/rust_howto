# Parallel Tasks

{{#include parallel.incl.md}}

## Mutate the elements of an array in parallel

[![rayon][c-rayon-badge]][c-rayon]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

The example uses the {{hi:rayon}}[`rayon`][c-rayon]⮳ crate, which is a {{i:data parallelism}} library for Rust.
{{hi:rayon}}[`rayon`][c-rayon]⮳ provides the {{hi:par_iter_mut}}[`par_iter_mut`][c-rayon::iter::IntoParallelRefIterator::par_iter_mut]⮳ method for any parallel iterable data type. This is an iterator-like chain that potentially executes in parallel.

```rust,editable
{{#include ../../../deps/tests/rayon-iter-mut.rs}}
```

## Test in parallel if any or all elements of a collection match a given predicate

[![rayon][c-rayon-badge]][c-rayon]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example demonstrates using the {{hi:rayon::any}}[`rayon::any`][c-rayon::iter::ParallelIterator::any]⮳ and {{hi:rayon::all}}[`rayon::all`][c-rayon::iter::ParallelIterator::any]⮳ methods, which are parallelized counterparts to {{hi:std::any}}[`std::any`][c-std::iter::Iterator::any]⮳ and {{hi:std::all}}[`std::all`][c-std::iter::Iterator::all]⮳. {{hi:rayon::any}}[`rayon::any`][c-rayon::iter::ParallelIterator::any]⮳ checks in parallel whether any element of the iterator matches the predicate, and returns as soon as one is found. {{hi:rayon::all}}[`rayon::all`][c-rayon::iter::ParallelIterator::any]⮳ checks in parallel whether all elements of the iterator match the predicate, and returns as soon as a non-matching element is found.

```rust,editable
{{#include ../../../deps/tests/rayon-any-all.rs}}
```

## Search items using given predicate in parallel

[![rayon][c-rayon-badge]][c-rayon]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example uses {{hi:rayon::find_any}}[`rayon::find_any`][c-rayon::iter::ParallelIterator::find_any]⮳ and {{hi:par_iter}}[`par_iter`][c-rayon::iter::ParallelIterator::find_any]⮳ to search a vector in parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure argument of {{hi:rayon::find_any}}[`rayon::find_any`][c-rayon::iter::ParallelIterator::find_any]⮳ {{hi:rayon}}[`rayon`][c-rayon]⮳ returns the first one found, not necessarily the first one.

Also note that the argument to the closure is a reference to a reference (`&&x`). See the discussion on {{hi:std::find}}[`std::find`][c-std::iter::Iterator::find]⮳ for additional details.

```rust,editable
{{#include ../../../deps/tests/rayon-parallel-search.rs}}
```

## Sort a vector in parallel

[![rayon][c-rayon-badge]][c-rayon]  [![rand][c-rand-badge]][c-rand]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example will {{i:sort in parallel}} a vector of Strings.

Allocate a vector of empty Strings. `par_iter_mut().for_each` populates random values in parallel. Although [multiple options][c-rayon::slice::ParallelSliceMut]⮳
exist to sort an enumerable data type, {{hi:par_sort_unstable}}[`par_sort_unstable`][c-rayon::slice::ParallelSliceMut::par_sort_unstable]⮳ is usually faster than [stable sort][c-rayon::slice::ParallelSliceMut::par_sort] ⮳ algorithms.

```rust,editable
{{#include ../../../deps/tests/rayon-parallel-sort.rs}}
```

## Map-reduce in parallel

[![rayon][c-rayon-badge]][c-rayon]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example uses {{hi:rayon::filter}}[`rayon::filter`][c-rayon::iter::ParallelIterator::filter]⮳ {{hi:rayon::map}}[`rayon::map`][c-rayon::iter::ParallelIterator::map]⮳ and {{hi:rayon::reduce}}[`rayon::reduce`][c-rayon::iter::ParallelIterator::reduce]⮳ to calculate the average age of `Person` objects whose age is over 30.

{{hi:rayon::filter}}[`rayon::filter`][c-rayon::iter::ParallelIterator::filter]⮳ returns elements from a collection that satisfy the given predicate. {{hi:rayon::map}}[`rayon::map`][c-rayon::iter::ParallelIterator::map]⮳ performs an operation on every element, creating a new iteration, and {{hi:rayon::reduce}}[`rayon::reduce`][c-rayon::iter::ParallelIterator::reduce]⮳ performs an operation given the previous reduction and the current element. Also shows use of {{hi:rayon::sum}}[`rayon::sum`][c-rayon::iter::ParallelIterator::sum]⮳ which has the same result as the reduce operation in this example.

```rust,editable
{{#include ../../../deps/tests/rayon-map-reduce.rs}}
```

## Generate jpg thumbnails in parallel

[![rayon][c-rayon-badge]][c-rayon]  [![glob][c-glob-badge]][c-glob]  [![image][c-image-badge]][c-image]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all `jpg`{{hi:jpg}} files in the current directory then saves them in a new folder called `thumbnails`.

[`glob::{{i:glob_with}}`][c-glob::glob_with]⮳ finds jpeg files in current directory. {{hi:rayon}}[`rayon`][c-rayon] resizes images in parallel using {{hi:par_iter}}[`par_iter`][c-rayon::iter::IntoParallelRefIterator::par_iter]⮳ calling {{hi:DynamicImage::resize}}[`DynamicImage::resize`][c-image::DynamicImage::resize]⮳

```rust,editable,no_run
{{#include ../../../deps/tests/rayon-thumbnails.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
