# Sorting Vectors

{{#include sorting.incl.md}}

## Sort a Vector of Integers

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

This example will sort{{hi:Sorting}} a Vector of integers via [`std::vec::Vec::sort`][c-std::vec::Vec::sort]{{hi:std::vec::Vec::sort}}⮳. Alternative would be to use [`std::vec::Vec::sort_unstable`][c-std::vec::Vec::sort_unstable]{{hi:std::vec::Vec::sort_unstable}}⮳ which can be faster, but does not preserve the order of equal elements.

```rust
{{#include ../../../deps/tests/sort.rs}}
```

## Sort a Vector of Floats

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

A vector{{hi:Vector}} of f32 or f64 can be sorted with `sort_by` and [`std::cmp::PartialOrd::partial_cmp`][c-std::cmp::PartialOrd::partial_cmp]{{hi:std::cmp::PartialOrd::partial_cmp}}⮳.

```rust
{{#include ../../../deps/tests/sort_float.rs}}
```

## Sort a Vector of Structs

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

Sorts a vector{{hi:Vector}} of Person structs with properties `name` and `age` by its natural order (by name and age). In order to make `Person` sortable{{hi:Sorting}} you need four traits [`std::cmp::Eq`][c-std::cmp::Eq]{{hi:std::cmp::Eq}}⮳, [`std::cmp::PartialEq`][c-std::cmp::PartialEq]{{hi:std::cmp::PartialEq}}⮳, [`std::cmp::Ord`][c-std::cmp::Ord]{{hi:std::cmp::Ord}}⮳ and [`std::cmp::PartialOrd`][c-std::cmp::PartialOrd]{{hi:std::cmp::PartialOrd}}⮳. These traits can be simply derived. You can also provide a custom comparator function using a [`std::vec::Vec::sort_by`][c-std::vec::Vec::sort_by]{{hi:std::vec::Vec::sort_by}}⮳ method and sort only by age.

```rust
{{#include ../../../deps/tests/sort_struct.rs}}
```

## See also

- A Rust implementation of Glidesort,a stable adaptive quicksort/mergesort hybrid sorting algorithm: [glidesort][c-glidesort-github]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: organize
</div>
