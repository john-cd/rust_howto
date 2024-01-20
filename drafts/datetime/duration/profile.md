## Measure the elapsed time between two code sections

[![std-badge]][std] [![cat-time-badge]][cat-time]

Measures [`time::Instant::elapsed`] since [`time::Instant::now`].

Calling [`time::Instant::elapsed`] returns a [`time::Duration`] that we print at the end of the example.
This method will not mutate or reset the [`time::Instant`] object.

```rust,editable
{#include ../../../deps/examples/profile.rs}
```

[`time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[`time::Instant::elapsed`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed
[`time::Instant::now`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`time::Instant`]:https://doc.rust-lang.org/std/time/struct.Instant.html
