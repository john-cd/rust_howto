# Multithreading

## Spawn, join

```rust,editable
{{#include ../../deps/examples/multithreading_spawn_join.rs}}
```

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.

## Scoped threads

[Scoped threads]( https://doc.rust-lang.org/std/thread/fn.scope.html )

```rust,editable
{{#include ../../deps/examples/multithreading_scoped_threads.rs}}
```

## Rayon - parallel processing

[Rayon docs]( https://docs.rs/rayon/latest/rayon/ )

[Rayon github]( https://github.com/rayon-rs/rayon )

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

Rayon implements [join]( https://docs.rs/rayon/latest/rayon/fn.join.html ), [scope]( https://docs.rs/rayon/latest/rayon/fn.scope.html ), [spawn]( https://docs.rs/rayon/latest/rayon/fn.spawn.html ) that may run on the global or a custom [Rayon threadpool]( https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html# ).

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/multithreading_rayon_custom.rs}}
```
