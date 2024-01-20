### Standard deviation

[![std-badge]][std] [![cat-science-badge]][cat-science]

This example calculates the standard deviation and z-score of a set of measurements.

The standard deviation is defined as the square root of the variance (here calculated with f32's [`sqrt`], where the variance is the [`sum`] of the squared difference between each measurement and the [`mean`], divided by the number of measurements.

The z-score is the number of standard deviations a single measurement spans away from the [`mean`] of the data set.

```rust,editable
{#include ../../../deps/examples/standard-deviation.rs}
```

[sqrt]: https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt
[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[mean]: science/mathematics/statistics/central-tendency.html
