# Explicit threads

{{#include explicit_threads.incl.md}}

## Use spawn & join {#spawn-join}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Join}}

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/explicit_threads/multithreading_spawn_join.rs:example}}
```

Note: when the main thread of a Rust program completes, all spawned threads{{hi:Spawned threads}} are shut down, whether or not they have finished running.

## Use scoped threads {#scoped-threads}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[Scoped threads][c-std::thread::scope]⮳

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/explicit_threads/multithreading_scoped_threads.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[explicit_threads: polish (P1)](https://github.com/john-cd/rust_howto/issues/262)
</div>
