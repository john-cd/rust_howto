## Generate random numbers

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates random numbers with help of random-number
generator [`rand::Rng`] obtained via [`rand::thread_rng`]. Each thread has an
initialized generator. Integers are uniformly distributed over the range of the
type, and floating point numbers are uniformly distributed from 0 up to but not
including 1.

```rust,editable
{{#include ../../../deps/examples/rand.rs}}
```

[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::thread_rng`]: https://docs.rs/rand/*/rand/fn.thread_rng.html

{{#include ../../refs/link-refs.md}}
