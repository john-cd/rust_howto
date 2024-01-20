## Sort a Vector of Structs

[![std-badge]][std] [![cat-science-badge]][cat-science]

Sorts a Vector of Person structs with properties `name` and `age` by its natural
order (By name and age). In order to make Person sortable you need four traits [`Eq`],
[`PartialEq`], [`Ord`] and [`PartialOrd`]. These traits can be simply derived.
You can also provide a custom comparator function using a [`vec:sort_by`] method and sort only by age.

```rust,editable
{#include ../../../deps/examples/sort_struct.rs}
```

[`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[`Ord`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`vec:sort_by`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
