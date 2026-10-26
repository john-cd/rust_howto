<!-- TODO Reviewed ??-??-2026 -->

# Data Parallelism with Rayon

{{#include data_parallelism.incl.md}}

## Use Rayon for Parallel Iteration {#rayon}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

Rayon provides work-stealing parallel iterators and task primitives on a managed thread pool. Use it for CPU-bound work that can be split into independent tasks; measure the result because parallel overhead can outweigh the work for small inputs.

### Apply a Parallel Iterator {#par-iter}

Convert an iterator to `par_iter`, `par_iter_mut`, or `into_par_iter` when each item can be processed independently.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon.rs:example}}
```

### Split Independent Tasks with `rayon::join` {#custom-parallel-tasks}

`rayon::join` runs two independent computations and returns both results; Rayon may execute one branch on another worker.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon_custom.rs:example}}
```

## Mutate a Collection in Parallel {#mutate-array-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

Use `par_iter_mut` to update distinct elements through a parallel iterator while Rust keeps each mutable borrow disjoint.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_iter_mut.rs:example}}
```

## Test a Predicate Across a Collection {#any-or-all-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

Rayon's `any` short-circuits when it finds a match, while `all` stops at the first non-match.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_any_all.rs:example}}
```

## Find a Matching Item in Parallel {#search-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

`find_any` returns a matching item found by any worker, so it does not promise the first match in sequential iteration order.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_parallel_search.rs:example}}
```

## Sort Collections in Parallel {#parallel-sorting}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

Choose a stable sort when equal items must retain their input order; use the unstable variant when that ordering is unnecessary.

### Sort a Slice with `par_sort` {#sort-in-parallel}

The stable parallel sort keeps the relative order of equal values.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon_parsort.rs:example}}
```

### Sort a Vector with `par_sort_unstable` {#sort-vector-unstable}

This example fills a vector with generated strings and sorts it in parallel without preserving the order of equal values.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_parallel_sort.rs:example}}
```

## Map and Reduce Parallel Results {#map-reduce-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

Filter items, map them to partial results, and combine those values with `reduce` or a specialized operation such as `sum`.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_map_reduce.rs:example}}
```

## Generate JPEG Thumbnails in Parallel {#thumbnails-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![glob][c~glob~docs~badge]][c~glob~docs] [![image][c~image~docs~badge]][c~image~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

This example reads `images/*.jpg` and writes resized images under `temp/thumbnails`. Its block is `noplayground` because it expects local image files; tests create small JPEG fixtures in temporary directories. The `image` dependency enables only the JPEG and PNG codecs used by the examples.

[`glob::glob_with::glob_with`][c~glob::glob_with~docs]↗{{hi:glob::glob_with::glob_with}} finds jpeg files in current directory. [`rayon`][c~rayon~docs]↗{{hi:rayon}} resizes images in parallel using [`rayon::iter::IntoParallelRefIterator::par_iter`][c~rayon::iter::IntoParallelRefIterator::par_iter~docs]↗{{hi:rayon::iter::IntoParallelRefIterator::par_iter}} calling [`image::DynamicImage::resize`][c~image::DynamicImage::resize~docs]↗{{hi:image::DynamicImage::resize}}

```rust,editable,noplayground
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Explicit threads](explicit_threads.md) and [thread pools](threadpool.md)
- [Message passing](message_passing.md) and [shared state](shared_state.md)
- [SIMD and processor features](../hardware-support/processor.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  data_parallelism: polish; dedupe with multithreading.md
</div>
