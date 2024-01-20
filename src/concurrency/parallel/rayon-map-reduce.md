## Map-reduce in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::filter`], [`rayon::map`], and [`rayon::reduce`]
to calculate the average age of `Person` objects whose age is over 30.

[`rayon::filter`] returns elements from a collection that satisfy the given
predicate.  [`rayon::map`] performs an operation on every element, creating a
new iteration, and [`rayon::reduce`] performs an operation given the previous
reduction and the current element.  Also shows use of [`rayon::sum`],
which has the same result as the reduce operation in this example.

```rust,editable
{#include ../../../deps/examples/rayon-map-reduce.rs}
```

[`rayon::filter`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.filter
[`rayon::map`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.map
[`rayon::reduce`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.reduce
[`rayon::sum`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.sum
