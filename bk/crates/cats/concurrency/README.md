<!-- TODO Reviewed ??-??-2026 -->

# Concurrency

This crate contains code examples for the **Concurrency** category on crates.io. It demonstrates integration with key ecosystem crates including `actix`, `anyhow`, `await-tree`, `clap`, `crossbeam`, `crossbeam-channel`.

## Examples Covered

- **Actors** (`examples/actors/`): Demonstrates concepts like `actix`, `actors`, `ractor`, `riker`, `stakker`.
- **Atomics** (`examples/atomics/`): Demonstrates concepts like `atomic_cell`, `atomics`, `spinlock`.
- **Loom model** (`examples/loom_model.rs`): Model-checks a publication and cancellation protocol with `loom`.
- **Concurrent Data Structures** (`examples/concurrent_data_structures/`): Demonstrates concepts like `crossbeam_queue`, `dashmap`, `flurry`, `papaya`.
- **Crossbeam** (`examples/crossbeam/`): Demonstrates concepts like `crossbeam_complex`, `crossbeam_select`.
- **Data Parallelism** (`examples/data_parallelism/`): Demonstrates concepts like `multithreading_rayon`, `multithreading_rayon_custom`, `multithreading_rayon_parsort`, `rayon_any_all`, `rayon_iter_mut`, `rayon_map_reduce`, `rayon_parallel_search`, `rayon_parallel_sort`, `rayon_thumbnails`.
- **Explicit Threads** (`examples/explicit_threads/`): Demonstrates concepts like `multithreading_scoped_threads`, `multithreading_spawn_join`, `thread_builder`.
- **Message Passing** (`examples/message_passing/`): Demonstrates concepts like `flume`, `flume_commands`, `flume_tick`, `message_passing_crossbeam_channel`, `message_passing_crossbeam_channel_after_tick`, `message_passing_mpsc`.
- **Send Sync** (`examples/send_sync/`): Demonstrates concepts like `send_sync_impl`.
- **Shared State** (`examples/shared_state/`): Demonstrates concepts like `arc_make_mut`, `global_mut_state`, `send_sync`, `shared_state_mutex`, `shared_state_parking_lot`, `shared_state_parking_lot2`.
- **Spsc** (`examples/spsc/`): Demonstrates concepts like `ringbuf`, `rtrb`.
- **Threadpool** (`examples/threadpool/`): Demonstrates concepts like `threadpool_fractal`, `threadpool_walk`.
- **Standalone Examples**: `await_tree`.

## Usage

To check or run examples in this crate:

```console
cargo check --package concurrency --examples
cargo test --package concurrency
```

Code examples correspond to chapters in `bk/src/categories/concurrency` or related book sections.
