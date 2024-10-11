# Sorting Vectors

{{#include randomness.incl.md}}

## Sort a Vector of Integers

[![std][c-std-badge]][c-std]  [![cat-science][cat-science-badge]][cat-science]

This example will {{i:sort}} a Vector of integers via {{hi:vec::sort}}[`vec::sort`][c-std::vec::Vec::sort]⮳. Alternative would be to use {{hi:vec::sort_unstable}}[`vec::sort_unstable`][c-std::vec::Vec::sort_unstable]⮳ which can be faster, but does not preserve the order of equal elements.

```rust,editable
{{#include ../../../deps/tests/sort.rs}}
```

## Sort a Vector of Floats

[![std][c-std-badge]][c-std]  [![cat-science][cat-science-badge]][cat-science]

A {{i:vector}} of f32 or f64 can be sorted with {{hi:vec::sort_by}}[`vec::sort_by`][primitive-slice::sort_by] and {{hi:PartialOrd::partial_cmp}}[`PartialOrd::partial_cmp`][c-std::cmp::PartialOrd::partial_cmp]⮳.

```rust,editable
{{#include ../../../deps/tests/sort_float.rs}}
```

## Sort a Vector of Structs

[![std][c-std-badge]][c-std]  [![cat-science][cat-science-badge]][cat-science]

Sorts a {{i:vector}} of Person structs with properties `name` and `age` by its natural order (By name and age). In order to make `Person` {{i:sortable}} you need four traits {{hi:Eq}}[`Eq`][c-std::cmp::Eq]⮳, {{hi:PartialEq}}[`PartialEq`][c-std::cmp::PartialEq]⮳, {{hi:Ord}}[`Ord`][c-std::cmp::Ord]⮳ and {{hi:PartialOrd}}[`PartialOrd`][c-std::cmp::PartialOrd]⮳. These traits can be simply derived. You can also provide a custom comparator function using a {{hi:vec:sort_by}}[`vec:sort_by`][c-std::vec::Vec::sort_by]⮳ method and sort only by age.

```rust,editable
{{#include ../../../deps/tests/sort_struct.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
