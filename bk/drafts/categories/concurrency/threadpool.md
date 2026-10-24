<!-- TODO Reviewed ??-??-2026 -->

# Thread Pools

{{#include threadpool.incl.md}}

## Hash ISO Files in Parallel {#sha256-of-iso-files-concurrently}

[![threadpool][c~threadpool~docs~badge]][c~threadpool~docs]{{hi:threadpool}} [![num_cpus][c~num_cpus~docs~badge]][c~num_cpus~docs]{{hi:num_cpus}} [![walkdir][c~walkdir~docs~badge]][c~walkdir~docs]{{hi:walkdir}} [![ring][c~ring~docs~badge]][c~ring~docs]{{hi:ring}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] [{{hi:Concurrency}}![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

This recipe walks the current directory, submits `.iso` files to a fixed-size pool, and computes a SHA-256 digest with `ring`. The worker count comes from `num_cpus::get`; the example requires matching ISO files in its working directory.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/threadpool/threadpool_walk.rs:example}}
```

## Render a Fractal with a Thread Pool {#draw-fractal-threadpool}

[![threadpool][c~threadpool~docs~badge]][c~threadpool~docs]{{hi:threadpool}} [![num][c~num~docs~badge]][c~num~docs]{{hi:num}} [![num_cpus][c~num_cpus~docs~badge]][c~num_cpus~docs]{{hi:num_cpus}} [![image][c~image~docs~badge]][c~image~docs]{{hi:image}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~science][cat~science~badge]][cat~science]{{hi:science}} [![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}}

This recipe divides Julia-set pixels into jobs, renders them with the `threadpool` crate, and saves the resulting image. Use a pool for many independent jobs; Rayon is often a simpler fit when the work is over Rust collections.

[![julia set][web-julia-set]][web-julia-set]

The example uses `image::ImageBuffer` to store RGB pixels and an MPSC channel to return completed jobs to the main thread, which writes `output.png`.

```rust,editable,noplayground
{{#include ../../../crates/cats/concurrency/examples/threadpool/threadpool_fractal.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Shared state](shared_state.md) for synchronized mutable data.
- [Message passing](message_passing.md) for channels between workers.
- [Data parallelism with Rayon](data_parallelism.md#rayon) for parallel iterators and custom pools.
- [Actors](_actors.md) for message-driven workers.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  The fractal recipe remains `noplayground` because its native `cc` dependency failed to link in a previous validation; issue #268 owns that example.
</div>
