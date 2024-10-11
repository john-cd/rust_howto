# Multithreading

{{#include multithreading.incl.md}}

## Spawn, join

[![std][c-std-badge]][c-std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] {{hi:Join}}

```rust,editable
{{#include ../../../deps/tests/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all spawned threads{{hi:spawned threads}} are shut down, whether or not they have finished running.

## Scoped threads

[![std][c-std-badge]][c-std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[Scoped threads][c-std::thread::scope]⮳

```rust,editable
{{#include ../../../deps/tests/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[![rayon][c-rayon-badge]][c-rayon]  [![rayon-github][c-rayon-github-badge]][c-rayon-github]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel{{hi:execute in parallel}}.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon.rs}}
```

### Parallel sorting

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_parsort.rs}}
```

### Custom parallel tasks{{hi:parallel tasks}}

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Rayon implements [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::spawn`][c-rayon::spawn]{{hi:rayon::spawn}}⮳ that may run on the global or a custom [Rayon threadpool][c-rayon::join]{{hi:Threadpool}}⮳.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_custom.rs}}
```

## See also

[![threadpool][c-threadpool-badge]][c-threadpool]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
