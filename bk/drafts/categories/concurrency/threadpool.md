<!-- TODO Reviewed ??-??-2026 -->

# Thread Pools

{{#include threadpool.incl.md}}

## Hash ISO Files in Parallel {#sha256-of-iso-files-concurrently}

[![threadpool][c~threadpool~docs~badge]][c~threadpool~docs]{{hi:threadpool}} [![num_cpus][c~num_cpus~docs~badge]][c~num_cpus~docs]{{hi:num_cpus}} [![walkdir][c~walkdir~docs~badge]][c~walkdir~docs]{{hi:walkdir}} [![sha2][c~sha2~docs~badge]][c~sha2~docs]{{hi:sha2}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] [{{hi:Concurrency}}![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

This recipe walks the current directory, submits `.iso` files to a fixed-size pool, and computes SHA-256 digests with the pure-Rust `sha2` crate. The worker count comes from `num_cpus::get`; the example requires matching ISO files in its working directory.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/threadpool/threadpool_walk.rs:example}}
```

## Render a Fractal with Rayon {#draw-fractal-rayon}

[![rayon][c~rayon~docs~badge]][c~rayon~docs]{{hi:rayon}} [![num][c~num~docs~badge]][c~num~docs]{{hi:num}} [![image][c~image~docs~badge]][c~image~docs]{{hi:image}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~science][cat~science~badge]][cat~science]{{hi:science}} [![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}}

This recipe renders independent Julia-set rows with Rayon and saves the resulting image. Rayon is a good fit for parallel work over Rust collections.

[![julia set][web-julia-set]][web-julia-set]

The example uses `image::ImageBuffer` to store RGB pixels and Rayon to render rows in parallel. It writes `temp/output.png` using the PNG-only image codec configuration shared with the thumbnail examples.

The book excerpt hides the fractal math helpers outside the anchor, so it is not a complete standalone Playground program.

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
