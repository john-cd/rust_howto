## Search items using given predicate in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::find_any`] and [`par_iter`] to search a vector in
parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure
argument of [`rayon::find_any`], `rayon` returns the first one found, not
necessarily the first one.

Also note that the argument to the closure is a reference to a reference
(`&&x`). See the discussion on [`std::find`] for additional details.

```rust,editable
{#include ../../../deps/examples/rayon-parallel-search.rs}
```

[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`rayon::find_any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.find_any
[`std::find`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
