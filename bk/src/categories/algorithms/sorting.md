<!-- TODO Reviewed ??-??-2026. -->

# Sorting

{{#include sorting.incl.md}}

The standard library provides sorting methods on mutable slices (`&mut [T]`). You can call them directly on a mutable vector or array.

- `slice::sort()` is stable: equal elements keep their original relative order. Its current implementation is driftsort, which adapts to existing order in the input.
- `slice::sort_unstable()` may reorder equal elements. Its current implementation is based on ipnsort and sorts in place.
- `slice::sort_by_key()` sorts a slice based on a key extracted from each element.
- `slice::sort_by()` allows you to provide a custom comparison function for more complex sorting logic.

Choose between `sort()` and `sort_unstable()` based on whether equal elements must keep their relative order. The standard library may change its sorting implementations, so code should depend on these documented ordering guarantees rather than a particular algorithm.

Note the following crates and modules:

- `itertools` provides an `itertools::sorted` function and a `sorted()` method on its `Itertools` trait. It sorts an iterator into a new `Vec`, so no additional collection is needed: `(0..10).rev().sorted();`
- [`std::cmp`][c~std::cmp~docs]↗{{hi:std::cmp}} provides ordering traits.

## Sort a Vector of Integers {#sort-vector-integers}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:Sorting}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

This example sorts a vector of integers via [`sort`][c~std::vec::Vec::sort~docs]↗{{hi:std::vec::Vec::sort}}. Alternatively, use [`sort_unstable`][c~std::vec::Vec::sort_unstable~docs]↗{{hi:std::vec::Vec::sort_unstable}}, which can be faster but does not preserve the order of equal elements.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_basic.rs:example}}
```

## Sort a Vector of Floats {#sort-vector-floats}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:Sorting}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

`sort` and `sort_unstable` require the elements to implement the [`std::cmp::Ord`][c~std::cmp::Ord~docs]↗{{hi:std::cmp::Ord}} trait, which provides a total ordering.

Floats do not implement `Ord` because they can be `NaN`, but they implement the [`std::cmp::PartialOrd`][c~std::cmp::PartialOrd~docs]↗{{hi:std::cmp::PartialOrd}} trait. For a total order that handles every `f32` or `f64` value, including NaN and signed zero, use [`f64::total_cmp`][f64::total_cmp~docs]↗ (or `f32::total_cmp`). If the values are guaranteed not to contain NaN, `partial_cmp` is also suitable.

A vector{{hi:Vector}} of [`f32`][primitive~f32]↗{{hi:f32}} or [`f64`][primitive~f64]↗{{hi:f64}} values can therefore be sorted with [`sort_by`][c~std::vec::Vec::sort_by~docs]↗{{hi:sort_by}} and `total_cmp`:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_float.rs:example}}
```

## Sort a Vector of Structs {#sort-vector-structs}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}{{hi:Sorting}}

The example sorts a vector{{hi:Vector}} of `Person` values. Deriving `PartialEq`, `Eq`, `PartialOrd`, and [`Ord`][c~std::cmp::Ord~docs]↗ gives `Person` an ordering based on its fields. Because `name` is declared first, `sort()` compares names first and uses `age` to break ties.

To sort the oldest people first, the example uses `sort_by_key` with `Reverse(age)`. Its final [`sort_by`][c~std::vec::Vec::sort_by~docs]↗ comparison sorts names by their length in bytes and breaks ties by descending age.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_struct.rs:example}}
```

## See Also {#see-also .skip}

- [glidesort][c~glidesort~repo]↗ is a Rust implementation of Glidesort, a stable adaptive quicksort/mergesort hybrid sorting algorithm.

## Related Topics {#related-topics .skip}

- [[data-structures | Data Structures]].
- [[b-trees | B-trees]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
