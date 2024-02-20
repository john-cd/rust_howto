# Multithreading

## Spawn, join

[![std-badge]][std]

```rust,editable
{{#include ../../deps/tests/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.

## Scoped threads

[![std-badge]][std]

[Scoped threads][std::thread::scope]⮳

```rust,editable
{{#include ../../deps/tests/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[![rayon-badge]][rayon]  [![rayon-github][rayon-github-badge]][rayon-github]

### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel.

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/multithreading_rayon.rs}}
```

### Parallel sorting

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/multithreading_rayon_parsort.rs}}
```

### Custom parallel tasks

Rayon implements [`join`][rayon::join]⮳, [`scope`][rayon::scope]⮳, [`spawn`][rayon::spawn]⮳ that may run on the global or a custom [Rayon threadpool][rayon::ThreadPool]⮳.

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/multithreading_rayon_custom.rs}}
```

## See also

[![threadpool-badge]][threadpool]

{{#include ../refs/link-refs.md}}
