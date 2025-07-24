# cmp

{{#include cmp.incl.md}}

The `cmp` module provides traits for comparing values, which are essential for implementing algorithms that require ordering or equality checks. The most commonly used traits are:

- [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)⮳: This trait is used for types that can be compared for equality.
- [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)⮳: This trait is a marker trait that indicates a type has a reflexive equality relation, meaning `a == a` is always true.
- [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)⮳: This trait is used for types that can be compared for ordering, but may not have a total order.
- [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)⮳: This trait is used for types that have a total order, meaning every pair of values can be compared.

## Test for Equality with `PartialEq` and `Eq` {#eq}

[![std][c~std~docs~badge]][c~std~docs]

[`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)⮳, [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)⮳

```rust,editable
{{#include ../../crates/standard_library/examples/cmp/eq.rs:example}}
```

## Compare and Sort Values with `PartialOrd` and `Ord` {#ord}

[![std][c~std~docs~badge]][c~std~docs]

[`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)⮳, [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)⮳

```rust,editable
{{#include ../../crates/standard_library/examples/cmp/ord.rs:example}}
```

Sorting algorithms often rely on the `PartialOrd` and `Ord` trait to determine the order of elements. The standard library provides several sorting functions that use these traits, such as `sort()` and `sort_by()` on slices.

## Related Topics {#related-topics}

- [[algorithms | Algorithms]].
- [[data_structures | Data Structures]].
- [[generics | Generics]].
- [[sorting | Sorting]].
- [[traits | Traits]].

## References {#references}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
