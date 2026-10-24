<!-- TODO Reviewed ??-??-2026 -->

# Explicit Threads

{{#include explicit_threads.incl.md}}

## Spawn Threads and Collect Their Results {#spawn-join}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Join}}

A spawned thread returns a `JoinHandle`; call `join` when the result is needed and map a panic to an application error.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/explicit_threads/multithreading_spawn_join.rs:example}}
```

## Use Scoped Threads to Borrow Stack Data {#scoped-threads}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Scoped workers may borrow local data because the scope waits for them to finish before returning. This example uses `std::thread::scope`, which is in the standard library; use `crossbeam::scope` only when a Crossbeam-specific API is also needed.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/explicit_threads/multithreading_scoped_threads.rs:example}}
```

## Name Threads and Set the Stack Size {#thread-builder}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Build a named worker with `std::thread::Builder` when thread names improve diagnostics or a task needs a non-default stack size. `Builder::spawn` reports operating-system creation errors instead of panicking.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/explicit_threads/thread_builder.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Data parallelism with Rayon](data_parallelism.md)
- [Thread pools](threadpool.md)
- [Message passing](message_passing.md)
- [Shared state](shared_state.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
