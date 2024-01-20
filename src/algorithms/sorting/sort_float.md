## Sort a Vector of Floats

[![std-badge]][std] [![cat-science-badge]][cat-science]

A Vector of f32 or f64 can be sorted with [`vec::sort_by`] and [`PartialOrd::partial_cmp`].

```rust,editable
{#include ../../../deps/examples/sort_float.rs}
```

[`vec::sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`PartialOrd::partial_cmp`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp
