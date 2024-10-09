# Sorting Vectors

{{#include randomness.incl.md}}

## Sort a Vector of Integers

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

This example will {{i:sort}} a Vector of integers via [`{{i:vec::sort}}`][std::vec::Vec::sort]⮳. Alternative would be to use [`{{i:vec::sort_unstable}}`][std::vec::Vec::sort_unstable]⮳ which can be faster, but does not preserve the order of equal elements.

```rust,editable
{{#include ../../../deps/tests/sort.rs}}
```

## Sort a Vector of Floats

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

A {{i:vector}} of f32 or f64 can be sorted with [`{{i:vec::sort_by}}`][slice::sort_by] and [`{{i:PartialOrd::partial_cmp}}`][std::cmp::PartialOrd::partial_cmp]⮳.

```rust,editable
{{#include ../../../deps/tests/sort_float.rs}}
```

## Sort a Vector of Structs

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

Sorts a {{i:vector}} of Person structs with properties `name` and `age` by its natural order (By name and age). In order to make `Person` {{i:sortable}} you need four traits [`{{i:Eq}}`][std::cmp::Eq]⮳, [`{{i:PartialEq}}`][std::cmp::PartialEq]⮳, [`{{i:Ord}}`][std::cmp::Ord]⮳ and [`{{i:PartialOrd}}`][std::cmp::PartialOrd]⮳. These traits can be simply derived. You can also provide a custom comparator function using a [`{{i:vec:sort_by}}`][std::vec::Vec::sort_by]⮳ method and sort only by age.

```rust,editable
{{#include ../../../deps/tests/sort_struct.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
