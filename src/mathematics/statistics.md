# Statistics

## Measures of central tendency

[![std-badge]][std] [![cat-science-badge]][cat-science]

These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an `[Option]` to be handled by the caller.

The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using `[sum]` and `[len]` to determine the total value and count of values respectively.

```rust,editable
{#include ../../deps/examples/central-tendency.rs}
```

The second example calculates the median using the quickselect algorithm, which avoids a full `[sort]` by sorting only partitions of the data set known to possibly contain the median. This uses `[cmp]` and `[Ordering]` to succinctly decide the next partition to examine, and `[split_at]` to choose an arbitrary pivot for the next partition at each step.

```rust,editable
{#include ../../deps/examples/central-tendency2.rs}
```

The final example calculates the  mode using a mutable `[HashMap]` to collect counts of each distinct integer from the set, using a `[fold]` and the `[entry]` API. The most frequent value in the `[HashMap]` surfaces with `[max_by_key]`

```rust,editable
{#include ../../deps/examples/central-tendency3.rs}
```

## Standard deviation

[![std-badge]][std] [![cat-science-badge]][cat-science]

This example calculates the standard deviation and z-score of a set of measurements.

The standard deviation is defined as the square root of the variance (here calculated with f32's `[sqrt]` where the variance is the `[sum]` of the squared difference between each measurement and the `[mean]` divided by the number of measurements.

The z-score is the number of standard deviations a single measurement spans away from the `[mean]` of the data set.

```rust,editable
{#include ../../deps/examples/standard-deviation.rs}
```

[sqrt]: https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt
[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[mean]: mathematics/statistics/central-tendency.html
[Option]: https://doc.rust-lang.org/std/option/enum.Option.html
[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[len]: https://doc.rust-lang.org/std/primitive.slice.html#method.len
[sort]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
[cmp]: https://doc.rust-lang.org/std/cmp/trait.Ord.html#tymethod.cmp
[Ordering]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html
[split_at]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at
[HashMap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[entry]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
[max_by_key]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key
{{#include ../refs/link-refs.md}}
