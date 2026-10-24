<!-- TODO Reviewed ??-??-2026 -->

# Statistics

{{#include statistics.incl.md}}

## Calculate Measures of Central Tendency {#measures-of-central-tendency}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [`std::option::Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}} to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [`std::iter::Iterator::sum`][c~std::iter::Iterator::sum~docs]↗{{hi:std::iter::Iterator::sum}} and [`len`][primitive~slice::len]↗{{hi:slice::len}} to determine the total value and count of values respectively.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency.rs:example}}
```

The second example calculates the median with a hand-written quickselect implementation. It recursively partitions into new `Vec` values; it does not sort those partitions, and it allocates while doing so. For an in-place selection, [`slice::select_nth_unstable`][primitive~slice::select_nth_unstable]↗{{hi:slice::select_nth_unstable}} is available in the standard library and has been stable since Rust 1.49.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency1.rs:example}}
```

The final example calculates the mode using a mutable [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}} to collect counts of each distinct integer from the set, using a [`std::iter::Iterator::fold`][c~std::iter::Iterator::fold~docs]↗{{hi:std::iter::Iterator::fold}} and the [`std::collections::hash_map::Entry`][c~std::collections::hash_map::Entry~docs]↗{{hi:std::collections::hash_map::Entry}} API. The most frequent value in the [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}} surfaces with [`std::iter::Iterator::max_by_key`][c~std::iter::Iterator::max_by_key~docs]↗{{hi:std::iter::Iterator::max_by_key}}. If values tie for the highest frequency, this implementation returns one according to hash-map iteration order; it does not define a deterministic tie-breaking rule.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency2.rs:example}}
```

## Compute the Standard Deviation {#standard-deviation}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

This example calculates the standard deviation and z-score of a set of measurements.

This example computes the population standard deviation: it divides the sum of squared differences from the mean by the number of measurements, then takes the square root with f64's [`sqrt`][primitive~f64::sqrt]↗{{hi:f64::sqrt}}. The sample standard deviation instead divides by `n - 1` and requires at least two observations.

The z-score is the number of standard deviations a single measurement spans away from the mean of the data set.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/standard_deviation.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[complex_numbers | Complex Numbers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
