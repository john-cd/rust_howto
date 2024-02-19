# Statistics

## Measures of central tendency

[![std-badge]][std] [![cat-science-badge]][cat-science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [`Option`][Option] to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [`sum`][sum] and [`len`][len] to determine the total value and count of values respectively.

```rust,editable
{{#include ../../deps/examples/central-tendency.rs}}
```

The second example calculates the median using the quickselect algorithm, which avoids a full [`sort`][sort] by sorting only partitions of the data set known to possibly contain the median. This uses [`cmp`][cmp] and [`Ordering`][Ordering] to succinctly decide the next partition to examine, and [`split_at`][split_at] to choose an arbitrary pivot for the next partition at each step.

```rust,editable
{{#include ../../deps/examples/central-tendency1.rs}}
```

The final example calculates the  mode using a mutable [`HashMap`][HashMap] to collect counts of each distinct integer from the set, using a [`fold`][fold] and the [`entry`][entry] API. The most frequent value in the [`HashMap`][HashMap] surfaces with [`max_by_key`][max_by_key]

```rust,editable
{{#include ../../deps/examples/central-tendency2.rs}}
```

## Standard deviation

[![std-badge]][std] [![cat-science-badge]][cat-science]

This example calculates the standard deviation and z-score of a set of measurements.

The standard deviation is defined as the square root of the variance (here calculated with f32's [`sqrt`][f32-sqrt] where the variance is the [`sum`][sum] of the squared difference between each measurement and the `mean` divided by the number of measurements).

The z-score is the number of standard deviations a single measurement spans away from the `mean` of the data set.

```rust,editable
{{#include ../../deps/examples/standard-deviation.rs}}
```

{{#include ../refs/link-refs.md}}
