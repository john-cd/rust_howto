# Comparing Values

{{#include cmp.incl.md}}

## Test for Equality with `PartialEq` and `Eq` {#eq}

[![std][c~std~docs~badge]][c~std~docs]

The [`std::cmp`][c~std::cmp~docs]↗{{hi:std::cmp}} module provides traits for comparing values and implementing algorithms that require ordering or equality checks.

- [`PartialEq`][c~std::cmp::PartialEq~docs]↗{{hi:std::cmp::PartialEq}} is used for types that can be checked for equality (using `==`{{hi:==}} and `!=`{{hi:!=}}).
- [`Eq`][c~std::cmp::Eq~docs]↗{{hi:std::cmp::Eq}} is a marker trait that indicates a type has a reflexive equality relation, meaning `a == a` is always true. It requires `PartialEq` to be implemented first.
- Floats (like [`f32`][primitive~f32]↗{{hi:f32}} and [`f64`][primitive~f64]↗{{hi:f64}}) do not implement `Eq`, because [`NaN`][f64::NAN~docs]↗{{hi:NaN}} != `NaN`. Beware when implementing equality on structs with a field of float type.

`PartialEq` and `Eq` are most often automatically derived using `#[derive(PartialEq, Eq)]` - see [[derive | Derive]]. You can however provide a custom implementation, if so desired:

```rust,editable
{{#include ../../crates/standard_library/examples/cmp/eq.rs:example}}
```

## Compare and Sort Values with `PartialOrd` and `Ord` {#ord}

[![std][c~std~docs~badge]][c~std~docs]

[`PartialOrd`][c~std::cmp::PartialOrd~docs]↗ is used for types that can be compared for ordering, but may not have a total order.

[`Ord`][c~std::cmp::Ord~docs]↗ is used for types that have a _total_ order, meaning every pair of values can be compared. `Ord` requires that the implementing type also be `PartialOrd`, [`PartialEq`][c~std::cmp::PartialEq~docs]↗{{hi:std::cmp::PartialEq}}, and [`Eq`][c~std::cmp::Eq~docs]↗{{hi:std::cmp::Eq}}.

Both traits can be automatically implemented with `#[derive(...)]`. When writing a custom implementation, it is recommended to read the documentation for `Ord` and `PartialOrd` to avoid logic errors: `Ord` must be consistent with the `PartialOrd` implementation, and `PartialOrd` with `PartialEq`.

Sorting algorithms often rely on the `PartialOrd` and `Ord` trait to determine the order of elements. The standard library provides several sorting functions that use these traits, such as [`sort()`][primitive~slice::sort]↗{{hi:slice::sort}} and [`sort_by()`][primitive~slice::sort_by]↗{{hi:slice::sort_by}} on slices.

The following example implements a custom order for software versions:

```rust,editable
{{#include ../../crates/standard_library/examples/cmp/ord.rs:example}}
```

## Related Topics {#related-topics}

- [[algorithms | Algorithms]].
- [[data_structures | Data Structures]].
- [[derive | Derive]].
- [[generics | Generics]].
- [[sorting | Sorting]].
- [[traits | Traits]].

## References {#references}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
