# Sorting Vectors

{{#include sorting.incl.md}}

## Sort a Vector of Integers {#sort-vector-integers}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}{{hi:Sorting}}

This example will sort a Vector of integers via [`std::vec::Vec::sort`][c~std::vec::Vec::sort~docs]{{hi:std::vec::Vec::sort}}↗. Alternative would be to use [`std::vec::Vec::sort_unstable`][c~std::vec::Vec::sort_unstable~docs]{{hi:std::vec::Vec::sort_unstable}}↗ which can be faster, but does not preserve the order of equal elements.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort1.rs:example}}
```

## Sort a Vector of Floats {#sort-vector-floats}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

A vector{{hi:Vector}} of f32 or f64 can be sorted with [`sort_by`][c~std::vec::Vec::sort_by~docs]↗{{hi:sort_by}} and [`std::cmp::PartialOrd::partial_cmp`][c~std::cmp::PartialOrd::partial_cmp~docs]{{hi:std::cmp::PartialOrd::partial_cmp}}↗.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_float.rs:example}}
```

## Sort a Vector of Structs {#sort-vector-structs}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

Sorts a vector{{hi:Vector}} of `Person` structs with properties `name` and `age` by its natural order (by name and age). In order to make `Person` sortable{{hi:Sorting}} you need four traits [`std::cmp::Eq`][c~std::cmp::Eq~docs]{{hi:std::cmp::Eq}}↗, [`std::cmp::PartialEq`][c~std::cmp::PartialEq~docs]{{hi:std::cmp::PartialEq}}↗, [`std::cmp::Ord`][c~std::cmp::Ord~docs]{{hi:std::cmp::Ord}}↗ and [`std::cmp::PartialOrd`][c~std::cmp::PartialOrd~docs]{{hi:std::cmp::PartialOrd}}↗. These traits can be simply derived. You can also provide a custom comparator function using a [`std::vec::Vec::sort_by`][c~std::vec::Vec::sort_by~docs]{{hi:std::vec::Vec::sort_by}}↗ method and sort only by age.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/sort/sort_struct.rs:example}}
```

## See Also {#see-also}

- [glidesort][c~glidesort~github]↗ is a Rust implementation of Glidesort, a stable adaptive quicksort/mergesort hybrid sorting algorithm.

## Related Topics {#related-topics}

- [[data-structures | Data Structures]].
- [[btrees | B-trees]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[sorting: organize](https://github.com/john-cd/rust_howto/issues/209)
</div>
