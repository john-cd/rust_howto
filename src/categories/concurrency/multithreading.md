# Multithreading

{{#include multithreading.incl.md}}

## Spawn, {{i:join}}

[![std][std-badge]][std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

```rust,editable
{{#include ../../../deps/tests/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all {{i:spawned threads}} are shut down, whether or not they have finished running.

## Scoped threads

[![std][std-badge]][std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[Scoped threads][c-std::thread::scope]⮳

```rust,editable
{{#include ../../../deps/tests/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[![rayon][rayon-badge]][rayon]  [![rayon-github][rayon-github-badge]][rayon-github]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to {{i:execute in parallel}}.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon.rs}}
```

### Parallel sorting

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_parsort.rs}}
```

### Custom {{i:parallel tasks}}

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Rayon implements [`{{i:join}}`][c-rayon::join]⮳, [`{{i:scope}}`][c-rayon::join]⮳, [`{{i:spawn}}`][c-rayon::spawn]⮳ that may run on the global or a custom [{{i:Rayon threadpool}}][c-rayon::join]⮳.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/multithreading_rayon_custom.rs}}
```

## See also

[![threadpool][threadpool-badge]][threadpool]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
