<!-- TODO Reviewed ??-??-2026 -->

Use shared state when threads need the same value. Prefer message passing when ownership can move through a channel. For shared mutation, start with `std::sync::Mutex`, `RwLock`, or atomics; the standard library is a good default. Reach for [`parking_lot`][c~parking_lot~docs], [`arc-swap`][c~arc-swap~docs], or [`DashMap`][c~dashmap~docs] when their specific behavior fits the workload.

| Recipe                                                                   | Crates                 | Categories                              |
| ------------------------------------------------------------------------ | ---------------------- | --------------------------------------- |
| [Share Immutable Ownership with `Arc`][ex~concurrency~arc]               | {{#crate std}}         | {{#categories concurrency=Concurrency}} |
| [Update Shared Values with `Arc::make_mut`][ex~concurrency~arc_make_mut] | {{#crate std}}         | {{#categories concurrency=Concurrency}} |
| [Maintain Global Mutable State][ex~concurrency~global-mutable-state]     | {{#crate std}}         | {{#categories concurrency=Concurrency}} |
| [Use a Mutex for Exclusive Mutation][ex~concurrency~mutex]               | {{#crate std}}         | {{#categories concurrency=Concurrency}} |
| [Share Mutable Data with `Arc<Mutex<T>>`][ex~concurrency~arc-mutex]      | {{#crate std}}         | {{#categories concurrency=Concurrency}} |
| [`parking_lot`][ex~concurrency~parking-lot]                              | {{#crate parking_lot}} | {{#categories concurrency=Concurrency}} |
