<!-- TODO Reviewed ??-??-2026 -->

Use shared state when threads need the same value. Prefer message passing when ownership can move through a channel. For shared mutation, start with `std::sync::Mutex`, `RwLock`, or atomics; the standard library is a good default. Reach for [`parking_lot`][c~parking_lot~docs], [`arc-swap`][c~arc-swap~docs], or [`DashMap`][c~dashmap~docs] when their specific behavior fits the workload.

| Recipe                                                                   | Crates                                                         | Categories                                                   |
| ------------------------------------------------------------------------ | -------------------------------------------------------------- | ------------------------------------------------------------ |
| [Share Immutable Ownership with `Arc`][ex~concurrency~arc]               | [![std][c~std~docs~badge]][c~std~docs]                         | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Update Shared Values with `Arc::make_mut`][ex~concurrency~arc_make_mut] | [![std][c~std~docs~badge]][c~std~docs]                         | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Maintain Global Mutable State][ex~concurrency~global-mutable-state]     | [![std][c~std~docs~badge]][c~std~docs]                         | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Use a Mutex for Exclusive Mutation][ex~concurrency~mutex]               | [![std][c~std~docs~badge]][c~std~docs]                         | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Share Mutable Data with `Arc<Mutex<T>>`][ex~concurrency~arc-mutex]      | [![std][c~std~docs~badge]][c~std~docs]                         | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [`parking_lot`][ex~concurrency~parking-lot]                              | [![parking_lot][c~parking_lot~docs~badge]][c~parking_lot~docs] | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
