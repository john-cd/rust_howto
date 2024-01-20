## Calculate SHA256 sum of iso files concurrently

[![threadpool-badge]][threadpool] [![num_cpus-badge]][num_cpus] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA256 for every file with iso extension in the
current directory. A threadpool generates threads equal to the number of cores
present in the system found with [`num_cpus::get`].  [`Walkdir::new`] iterates
the current directory and calls [`execute`] to perform the operations of reading
and computing SHA256 hash.

```rust,editable,no_run
{#include ../../../deps/examples/threadpool-walk.rs}
```

[`execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`Walkdir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new
