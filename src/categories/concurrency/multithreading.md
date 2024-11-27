# Multithreading

{{#include multithreading.incl.md}}

## Spawn, join {#spawn-join}

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Join}}

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/multithreading_spawn_join.rs:example}}
```

When the main thread of a Rust program completes, all spawned threads{{hi:Spawned threads}} are shut down, whether or not they have finished running.

## Scoped threads {#scoped-threads}

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[Scoped threads][c-std::thread::scope]⮳

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/multithreading_scoped_threads.rs:example}}
```

## Rayon - parallel processing {#rayon}

[![rayon][c-rayon-badge]][c-rayon]{{hi:rayon}}  [![rayon-github][c-rayon-github-badge]][c-rayon-github]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

### Parallel iteration {#par-iter}

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel{{hi:Parallel execution}}.

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/multithreading_rayon.rs:example}}
```

### Parallel sorting {#parallel-sorting}

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/multithreading_rayon_parsort.rs:example}}
```

### Custom parallel tasks {#custom-parallel-tasks}

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Parallel tasks}}

Rayon implements [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::join`][c-rayon::join]{{hi:rayon::join}}⮳, [`rayon::spawn`][c-rayon::spawn]{{hi:rayon::spawn}}⮳ that may run on the global or a custom [Rayon threadpool][c-rayon::join]{{hi:Thread pools}}⮳.

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/multithreading_rayon_custom.rs:example}}
```

## See also

[![threadpool][c-threadpool-badge]][c-threadpool]{{hi:threadpool}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 expand - threadpool?
</div>
