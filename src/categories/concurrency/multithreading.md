# Multithreading

{{#include multithreading.incl.md}}

## Spawn, join

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Join}}

```rust
{{#include ../../../deps/tests/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all spawned threads{{hi:Spawned threads}} are shut down, whether or not they have finished running.

## Scoped threads

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[Scoped threads][c-std::thread::scope]⮳

```rust
{{#include ../../../deps/tests/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[![rayon][c-rayon-badge]][c-rayon]{{hi:rayon}}  [![rayon-github][c-rayon-github-badge]][c-rayon-github]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel{{hi:Parallel execution}}.

```rust,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon.rs}}
```

### Parallel sorting

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

```rust,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_parsort.rs}}
```

### Custom parallel tasks

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Parallel tasks}}

Rayon implements [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::spawn`][c-rayon::spawn]{{hi:rayon::spawn}}⮳ that may run on the global or a custom [Rayon threadpool][c-rayon::join]{{hi:Thread pools}}⮳.

```rust,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_custom.rs}}
```

## See also

[![threadpool][c-threadpool-badge]][c-threadpool]{{hi:threadpool}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: expand - threadpool?
</div>
