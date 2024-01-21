# Sorting Vectors

## Sort a Vector of Integers

[![std-badge]][std] [![cat-science-badge]][cat-science]

This example sorts a Vector of integers via [`vec::sort`][vec::sort]. Alternative would be to use [`vec::sort_unstable`][vec::sort_unstable] which can be faster, but does not preserve the order of equal elements.

```rust,editable
{{#include ../../deps/examples/sort.rs}}
```

## Sort a Vector of Floats

[![std-badge]][std] [![cat-science-badge]][cat-science]

A Vector of f32 or f64 can be sorted with [`vec::sort_by`][vec::sort_by] and [`PartialOrd::partial_cmp`][PartialOrd::partial_cmp].

```rust,editable
{{#include ../../deps/examples/sort_float.rs}}
```

## Sort a Vector of Structs

[![std-badge]][std] [![cat-science-badge]][cat-science]

Sorts a Vector of Person structs with properties `name` and `age` by its natural
order (By name and age). In order to make Person sortable you need four traits [`Eq`][Eq], [`PartialEq`][PartialEq], [`Ord`][Ord] and [`PartialOrd`][PartialOrd]. These traits can be simply derived. You can also provide a custom comparator function using a [`vec:sort_by`][vec:sort_by] method and sort only by age.

```rust,editable
{{#include ../../deps/examples/sort_struct.rs}}
```

[vec::sort]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
[vec::sort_unstable]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
[vec::sort_by]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[PartialOrd::partial_cmp]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp
[Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[vec:sort_by]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
{{#include ../refs/link-refs.md}}
