## Maintain global mutable state

[![lazy-static-badge]][lazy-static] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declare global state using [lazy-static]. [lazy-static]
creates a globally available `static ref` which requires a [`Mutex`]
to allow mutation (also see [`RwLock`]). The [`Mutex`] wrap ensures
the state cannot be simultaneously accessed by multiple threads, preventing
race conditions. A [`MutexGuard`] must be acquired to read or mutate the
value stored in a [`Mutex`].

```rust,editable
{#include ../../../../deps/examples/global-mut-state.rs}
```

[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
