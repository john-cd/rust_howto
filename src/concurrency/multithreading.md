# Multithreading

## Spawn, join

```rust,editable
{{#include ../../deps/examples/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.

## Scoped threads

[Scoped threads][std::thread::scope]⮳

```rust,editable
{{#include ../../deps/examples/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[Rayon (docs)][rayon]⮳

[Rayon (github)][rayon-github]⮳

### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/multithreading_rayon.rs}}
```

### Parallel sorting

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/multithreading_rayon_parsort.rs}}
```

### Custom parallel tasks

Rayon implements [join][join]⮳, [scope][scope]⮳, [spawn][spawn]⮳ that may run on the global or a custom [Rayon threadpool][rayon-threadpool]⮳.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/multithreading_rayon_custom.rs}}
```

{{#include ../refs/link-refs.md}}
