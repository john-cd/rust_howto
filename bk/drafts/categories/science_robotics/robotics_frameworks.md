# Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

High-level robotics frameworks in pure Rust are still emerging.

## Zero Overhead Pub/Sub/Query Protocol with `zenoh` {#zenoh}

[![zenoh][c-zenoh-badge]][c-zenoh]{{hi:zenoh}}
[![zenoh-crates.io][c-zenoh-crates.io-badge]][c-zenoh-crates.io]
[![zenoh-github][c-zenoh-github-badge]][c-zenoh-github]
[![zenoh-lib.rs][c-zenoh-lib.rs-badge]][c-zenoh-lib.rs]

`Zenoh` - A high performance and extremely low overhead Pub/Sub/Query protocol. Quickly becoming the protocol of choice for Robot-to-Anything communication.

[Zenoh][c-zenoh-website]{{hi:zenoh}}⮳ is a zero-overhead Pub/Sub/Query protocol. Zenoh (pronounced as /zeno/) unifies data in motion, data at rest and computations. It blends traditional pub/sub with geo-distributed storages, queries and computations, while retaining time and space efficiency.

Zenoh is a great tool for data storage, query, and computations over geographically distributed systems.

Zenoh deals with keys/values where each key is a path and is associated to a value. A key looks like just a Unix file system path, such as `myhome/kitchen/temperature`. The value can be defined with different encodings (string, [JSON][p-json], raw bytes buffers).

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/zenoh.rs:example}}
```

## Open Rust Robotics {#open-rust-robotics}

[![openrr][c-openrr-badge]][c-openrr]{{hi:openrr}}
[![openrr-crates.io][c-openrr-crates.io-badge]][c-openrr-crates.io]
[![openrr-github][c-openrr-github-badge]][c-openrr-github]
[![openrr-lib.rs][c-openrr-lib.rs-badge]][c-openrr-lib.rs]

[`openrr`][c-openrr]⮳{{hi:openrr}} (Open Rust Robotics)

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/openrr.rs:example}}
```

## `copper`

Framework for creating fast and reliable robots.

`copper` is a user-friendly robotics framework designed for creating fast and reliable robots. Copper is to robots what a game engine is to games.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[useful_robotics_tools_and_libs: locate libs, organize, write](https://github.com/john-cd/rust_howto/issues/479)

## Frameworks

`dora-rs` - A fast and simple robotics frameworks for AI.
`Peng` - A minimal quadrotor autonomy framework

## gRPC

gRPC A high performance, open source universal RPC framework.

`CleanIt` - Open-source Autonomy Software in Rust-lang with gRPC for the Roomba series robot vacuum cleaners.

</div>
