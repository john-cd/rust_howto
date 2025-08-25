# Sorting

{{#include sorting.incl.md}}

## Sort a Vector of Integers {#sort-vector-integers}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:Sorting}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

This example sorts a vector of integers via [`sort`][c~std::vec::Vec::sort~docs]↗{{hi:std::vec::Vec::sort}}. Alternatively, you may use [`sort_unstable`][c~std::vec::Vec::sort_unstable~docs]↗{{hi:std::vec::Vec::sort_unstable}}, which can be faster but does not preserve the order of equal elements.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort1.rs:example}}
```

## Sort a Vector of Floats {#sort-vector-floats}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:Sorting}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

`sort` and `sort_unstable` require the elements to implement the [`std::cmp::Ord`][c~std::cmp::Ord~docs]↗{{hi:std::cmp::Ord}} trait, which provides a total ordering.

Floats do not implement `Ord` trait (because they can be `NaN`, "not a number"), but they implement the [`std::cmp::PartialOrd`][c~std::cmp::PartialOrd~docs]↗{{hi:std::cmp::PartialOrd}} trait, which provides partial ordering.

A vector{{hi:Vector}} of [`f32`][primitive~f32]↗{{hi:f32}} or [`f64`][primitive~f64]↗{{hi:f64}} guranteed not to contain `NaN` values can therefore be sorted using [`sort_by`][c~std::vec::Vec::sort_by~docs]↗{{hi:sort_by}} and [`std::cmp::PartialOrd::partial_cmp`][c~std::cmp::PartialOrd::partial_cmp~docs]↗{{hi:std::cmp::PartialOrd::partial_cmp}}.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_float.rs:example}}
```

## Sort a Vector of Structs {#sort-vector-structs}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}{{hi:Sorting}}

The following example sorts a vector{{hi:Vector}} of `Person` structs with properties `name` and `age` by its natural order (by name and age). In order to make the `Person` struct sortable, we need to implement four traits [`std::cmp::Eq`][c~std::cmp::Eq~docs]↗{{hi:std::cmp::Eq}}, [`std::cmp::PartialEq`][c~std::cmp::PartialEq~docs]↗{{hi:std::cmp::PartialEq}}, [`std::cmp::Ord`][c~std::cmp::Ord~docs]↗{{hi:std::cmp::Ord}} and [`std::cmp::PartialOrd`][c~std::cmp::PartialOrd~docs]↗{{hi:std::cmp::PartialOrd}}. These traits can be simply derived with an attribute, producing a lexicographic ordering based on the top-to-bottom declaration order of the struct's members.

To sort in a different way, provide a custom comparator function to the [`std::vec::Vec::sort_by`][c~std::vec::Vec::sort_by~docs]↗{{hi:std::vec::Vec::sort_by}} method:

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_struct.rs:example}}
```

## See Also {#see-also}

- [glidesort][c~glidesort~repo]↗ is a Rust implementation of Glidesort, a stable adaptive quicksort/mergesort hybrid sorting algorithm.

## Related Topics {#related-topics}

- [[data-structures | Data Structures]].
- [[btrees | B-trees]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
