# Statistics

{{#include statistics.incl.md}}

## Calculate Measures of Central Tendency {#measures-of-central-tendency}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}}↗ to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [`std::iter::Iterator::sum`][c~std::iter::Iterator::sum~docs]{{hi:std::iter::Iterator::sum}}↗ and [`len`][primitive~slice::len]{{hi:slice::len}}↗ to determine the total value and count of values respectively.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency.rs:example}}
```

The second example calculates the median using the quickselect algorithm, which avoids a full [`sort`][primitive~slice::sort]{{hi:slice::sort}}↗ by sorting only partitions of the data set known to possibly contain the median. This uses [`std::cmp::Ord::cmp`][c~std::cmp::Ord::cmp~docs]{{hi:std::cmp::Ord::cmp}}↗ and [`std::cmp::Ordering`][c~std::cmp::Ordering~docs]{{hi:std::cmp::Ordering}}↗ to succinctly decide the next partition to examine, and [`split_at`][primitive~slice::split_at]{{hi:split_at}}↗ to choose an arbitrary pivot for the next partition at each step.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency1.rs:example}}
```

The final example calculates the mode using a mutable [`std::collections::HashMap`][c~std::collections::HashMap~docs]{{hi:std::collections::HashMap}}↗ to collect counts of each distinct integer from the set, using a [`std::iter::Iterator::fold`][c~std::iter::Iterator::fold~docs]{{hi:std::iter::Iterator::fold}}↗ and the [`std::collections::hash_map::Entry`][c~std::collections::hash_map::Entry~docs]{{hi:std::collections::hash_map::Entry}}↗ API. The most frequent value in the [`std::collections::HashMap`][c~std::collections::HashMap~docs]{{hi:std::collections::HashMap}}↗ surfaces with [`std::iter::Iterator::max_by_key`][c~std::iter::Iterator::max_by_key~docs]{{hi:std::iter::Iterator::max_by_key}}↗.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/central_tendency2.rs:example}}
```

## Compute the Standard Deviation {#standard-deviation}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

This example calculates the standard deviation and z-score of a set of measurements.

The standard deviation is defined as the square root of the variance (here calculated with f32's [`sqrt`][primitive~f32::sqrt]{{hi:f32::sqrt}}↗ where the variance is the [`std::iter::Iterator::sum`][c~std::iter::Iterator::sum~docs]{{hi:std::iter::Iterator::sum}}↗ of the squared difference between each measurement and the `mean` divided by the number of measurements).

The z-score is the number of standard deviations a single measurement spans away from the `mean` of the data set.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/stats/standard_deviation.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/937)
</div>
