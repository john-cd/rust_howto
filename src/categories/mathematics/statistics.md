# Statistics

{{#include statistics.incl.md}}

## Measures of central tendency

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [`Option`][std::option::Option]⮳ to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [`sum`][sum]⮳ and [`len`][slice::len]⮳ to determine the total value and count of values respectively.

```rust,editable
{{#include ../../../deps/tests/central-tendency.rs}}
```

The second example calculates the median using the quickselect algorithm, which avoids a full [`sort`][slice::sort]⮳ by sorting only partitions of the data set known to possibly contain the median. This uses [`cmp`][std::cmp::Ord::cmp]⮳ and [`Ordering`][std::cmp::Ordering]⮳ to succinctly decide the next partition to examine, and [`split_at`][slice::split_at]⮳ to choose an arbitrary pivot for the next partition at each step.

```rust,editable
{{#include ../../../deps/tests/central-tendency1.rs}}
```

The final example calculates the mode using a mutable [`HashMap`][std::collections::HashMap]⮳ to collect counts of each distinct integer from the set, using a [`fold`][std::iter::Iterator::fold]⮳ and the [`entry`][std::collections::hash_map::Entry]⮳ API. The most frequent value in the [`HashMap`][std::collections::HashMap]⮳ surfaces with [`max_by_key`][std::iter::Iterator::max_by_key]⮳.

```rust,editable
{{#include ../../../deps/tests/central-tendency2.rs}}
```

## Standard deviation

[![std][std-badge]][std]  [![cat-science][cat-science-badge]][cat-science]

This example calculates the standard deviation and z-score of a set of measurements.

The standard deviation is defined as the square root of the variance (here calculated with f32's [`sqrt`][f32::sqrt]⮳ where the variance is the [`sum`][sum]⮳ of the squared difference between each measurement and the `mean` divided by the number of measurements).

The z-score is the number of standard deviations a single measurement spans away from the `mean` of the data set.

```rust,editable
{{#include ../../../deps/tests/standard-deviation.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
