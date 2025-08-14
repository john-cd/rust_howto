# Parallel Tasks

{{#include data_parallelism.incl.md}}

## `rayon` {#rayon}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Simple work-stealing parallelism for Rust using [`rayon`][c~rayon~docs]↗{{hi:rayon}}.

[`rayon`][c~rayon~docs]↗{{hi:rayon}} makes it easy to write parallel code. It provides data parallelism through [iterators][p~iterators] and task parallelism through scoped threads, allowing developers to convert sequential code to parallel versions with minimal changes. Rayon manages the thread pool and workload distribution, simplifying parallel programming and improving [performance][p~performance] on multi-core processors.

### Iterate in Parallel {#par-iter}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Convert calls to [`iter`][c~std::iter~docs]↗{{hi:std::iter}} or `iter_mut` or `into_iter` into `par_iter` or [`par_iter_mut`][c~rayon::iter::IntoParallelRefMutIterator::par_iter_mut~docs]↗{{hi:par_iter_mut}} or `into_par_iter` to execute in parallel{{hi:Parallel execution}}.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon.rs:example}}
```

### Sort in Parallel {#parallel-sorting}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`rayon`][c~rayon~docs]↗{{hi:rayon}} simplifies parallel sorting in Rust by providing parallel iterators and functions that can be used to sort collections concurrently.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon_parsort.rs:example}}
```

### Implement Custom Parallel Tasks {#custom-parallel-tasks}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Parallel tasks}}

[`rayon`][c~rayon~docs]↗{{hi:rayon}} implements [`rayon::join`][c~rayon::join~docs]↗{{hi:rayon::join}}, [`rayon::join`][c~rayon::join~docs]↗{{hi:rayon::join}}, [`rayon::spawn`][c~rayon::spawn~docs]↗{{hi:rayon::spawn}} that may run on the global or a custom [Rayon threadpool][c~rayon::join~docs]↗{{hi:Thread pools}}.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/multithreading_rayon_custom.rs:example}}
```

## Mutate the Elements of an Array in Parallel {#mutate-array-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

The example uses the [`rayon`][c~rayon~docs]↗{{hi:rayon}} crate, which is a data parallelism{{hi:Data parallelism}} library for Rust.
[`rayon`][c~rayon~docs]↗{{hi:rayon}} provides the [`rayon::iter::IntoParallelRefMutIterator::par_iter_mut`][c~rayon::iter::IntoParallelRefMutIterator::par_iter_mut~docs]↗{{hi:rayon::iter::IntoParallelRefMutIterator::par_iter_mut}} method for any parallel iterable data type. This is an iterator-like chain that potentially executes in parallel.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_iter_mut.rs:example}}
```

## Test in Parallel if Any or All Elements of a Collection Match a Given Predicate {#any-or-all-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

This example demonstrates using the [`rayon::iter::ParallelIterator::any`][c~rayon::iter::ParallelIterator::any~docs]↗{{hi:rayon::iter::ParallelIterator::any}} and [`rayon::iter::ParallelIterator::any`][c~rayon::iter::ParallelIterator::any~docs]↗{{hi:rayon::iter::ParallelIterator::any}} methods, which are parallelized counterparts to [`std::iter::Iterator::any`][c~std::iter::Iterator::any~docs]↗{{hi:std::iter::Iterator::any}} and [`std::iter::Iterator::all`][c~std::iter::Iterator::all~docs]↗{{hi:std::iter::Iterator::all}}. [`rayon::iter::ParallelIterator::any`][c~rayon::iter::ParallelIterator::any~docs]↗{{hi:rayon::iter::ParallelIterator::any}} checks in parallel whether any element of the iterator matches the predicate, and returns as soon as one is found. [`rayon::iter::ParallelIterator::any`][c~rayon::iter::ParallelIterator::any~docs]↗{{hi:rayon::iter::ParallelIterator::any}} checks in parallel whether all elements of the iterator match the predicate, and returns as soon as a non-matching element is found.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_any_all.rs:example}}
```

## Search Items Using a Given Predicate in Parallel {#search-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

This example uses [`rayon::iter::ParallelIterator::find_any`][c~rayon::iter::ParallelIterator::find_any~docs]↗{{hi:rayon::iter::ParallelIterator::find_any}} and [`rayon::iter::ParallelIterator::find_any`][c~rayon::iter::ParallelIterator::find_any~docs]↗{{hi:rayon::iter::ParallelIterator::find_any}} to search a vector in parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure argument of [`rayon::iter::ParallelIterator::find_any`][c~rayon::iter::ParallelIterator::find_any~docs]↗{{hi:rayon::iter::ParallelIterator::find_any}} [`rayon`][c~rayon~docs]↗{{hi:rayon}} returns the first one found, not necessarily the first one.

Also note that the argument to the closure is a reference to a reference (`&&x`). See the discussion on [`std::iter::Iterator::find`][c~std::iter::Iterator::find~docs]↗{{hi:std::iter::Iterator::find}} for additional details.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_parallel_search.rs:example}}
```

## Sort a Vector in Parallel {#sort-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

This example will sort in parallel{{hi:Parallel sort}} a vector of Strings.

Allocate a vector of empty [Strings][p~strings]. `par_iter_mut().for_each` populates random values in parallel. Although [multiple options][c~rayon::slice::ParallelSliceMut~docs]↗ exist to sort an enumerable data type, [`rayon::slice::ParallelSliceMut::par_sort_unstable`][c~rayon::slice::ParallelSliceMut::par_sort_unstable~docs]↗{{hi:rayon::slice::ParallelSliceMut::par_sort_unstable}} is usually faster than [stable sort][c~rayon::slice::ParallelSliceMut::par_sort~docs]↗ algorithms.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_parallel_sort.rs:example}}
```

## Map-reduce in Parallel {#map-reduce-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

This example uses [`rayon::iter::ParallelIterator::filter`][c~rayon::iter::ParallelIterator::filter~docs]↗{{hi:rayon::iter::ParallelIterator::filter}} [`rayon::iter::ParallelIterator::map`][c~rayon::iter::ParallelIterator::map~docs]↗{{hi:rayon::iter::ParallelIterator::map}} and [`rayon::iter::ParallelIterator::reduce`][c~rayon::iter::ParallelIterator::reduce~docs]↗{{hi:rayon::iter::ParallelIterator::reduce}} to calculate the average age of `Person` objects whose age is over 30.

[`rayon::iter::ParallelIterator::filter`][c~rayon::iter::ParallelIterator::filter~docs]↗{{hi:rayon::iter::ParallelIterator::filter}} returns elements from a collection that satisfy the given predicate. [`rayon::iter::ParallelIterator::map`][c~rayon::iter::ParallelIterator::map~docs]↗{{hi:rayon::iter::ParallelIterator::map}} performs an operation on every element, creating a new iteration, and [`rayon::iter::ParallelIterator::reduce`][c~rayon::iter::ParallelIterator::reduce~docs]↗{{hi:rayon::iter::ParallelIterator::reduce}} performs an operation given the previous reduction and the current element. Also shows use of [`rayon::iter::ParallelIterator::sum`][c~rayon::iter::ParallelIterator::sum~docs]↗{{hi:rayon::iter::ParallelIterator::sum}} which has the same result as the reduce operation in this example.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_map_reduce.rs:example}}
```

## Generate JPEG Thumbnails in Parallel {#thumbnails-in-parallel}

[![rayon][c~rayon~docs~badge]][c~rayon~docs] [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io] [![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]{{hi:rayon}}{{hi:Concurrency}}{{hi:Parallel}}{{hi:Thread}}{{hi:Performance}}{{hi:Join}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![glob][c~glob~docs~badge]][c~glob~docs]{{hi:glob}} [![image][c~image~docs~badge]][c~image~docs]{{hi:image}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

This example generates thumbnails for all `jpg`{{hi:JPEG}} files in the current directory then saves them in a new folder called [`thumbnails`][c~thumbnails~docs]↗{{hi:thumbnails}}.

[`glob::glob_with::glob_with`][c~glob::glob_with~docs]↗{{hi:glob::glob_with::glob_with}} finds jpeg files in current directory. [`rayon`][c~rayon~docs]↗{{hi:rayon}} resizes images in parallel using [`rayon::iter::IntoParallelRefIterator::par_iter`][c~rayon::iter::IntoParallelRefIterator::par_iter~docs]↗{{hi:rayon::iter::IntoParallelRefIterator::par_iter}} calling [`image::DynamicImage::resize`][c~image::DynamicImage::resize~docs]↗{{hi:image::DynamicImage::resize}}

```rust,editable,noplayground
{{#include ../../../crates/cats/concurrency/examples/data_parallelism/rayon_thumbnails.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data_parallelism: polish; dedupe with multithreading.md](https://github.com/john-cd/rust_howto/issues/260)
rayon_thumbnails.rs is noplayground - linking with [`cc`][c~cc~docs]↗{{hi:cc}} failed: exit status: 1 - fix?
</div>
