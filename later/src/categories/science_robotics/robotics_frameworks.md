<!-- TODO Reviewed ??-??-2026 -->

# Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

High-level robotics frameworks in pure Rust are still emerging. Zenoh and OpenRR source examples remain available, but their Cargo features and example registrations are commented out because their dependency trees include unresolved security advisories.

## Zero Overhead Pub/Sub/Query Protocol with `zenoh` {#zenoh}

[![zenoh][c~zenoh~docs~badge]][c~zenoh~docs]{{hi:zenoh}}
[![zenoh~crates.io][c~zenoh~crates.io~badge]][c~zenoh~crates.io]
[![zenoh~repo][c~zenoh~repo~badge]][c~zenoh~repo]
[![zenoh~lib.rs][c~zenoh~lib.rs~badge]][c~zenoh~lib.rs]

[`Zenoh`][c~zenoh~docs]↗{{hi:Zenoh}} - A high performance and extremely low overhead Pub/Sub/Query protocol. Quickly becoming the protocol of choice for Robot-to-Anything communication.

[Zenoh][c~zenoh~website]↗{{hi:zenoh}} is a zero-overhead Pub/Sub/Query protocol. Zenoh (pronounced as /zeno/) unifies data in motion, data at rest and computations. It blends traditional pub/sub with geo-distributed storages, queries and computations, while retaining time and space efficiency.

Zenoh is a tool for data storage, queries, and computations over geographically distributed systems. The preserved example publishes and receives local telemetry, but is not compiled while Zenoh requires affected `lz4_flex 0.10`:

```rust,ignore
{{#include ../../../crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs:example}}
```

Zenoh deals with keys/values where each key is a path and is associated to a value. A key looks like just a Unix file system path, such as `myhome/kitchen/temperature`. The value can be defined with different encodings (string, [JSON][p~json], raw bytes buffers).

## Open Rust Robotics {#open-rust-robotics}

[![openrr][c~openrr~docs~badge]][c~openrr~docs]{{hi:openrr}}
[![openrr~crates.io][c~openrr~crates.io~badge]][c~openrr~crates.io]
[![openrr~repo][c~openrr~repo~badge]][c~openrr~repo]
[![openrr~lib.rs][c~openrr~lib.rs~badge]][c~openrr~lib.rs]

[`openrr`][c~openrr~docs]↗{{hi:openrr}} (Open Rust Robotics)

The preserved Rust source is still only a placeholder. Its dependency tree includes affected `fast-float` and `serde_with` releases, so the Cargo feature and example registration remain commented out until those dependencies can be updated.

## `copper` {#copper}

Framework for creating fast and reliable robots.

[`copper`][c~copper~crates.io]↗{{hi:copper}} is a user-friendly robotics framework designed for creating fast and reliable robots. Copper is to robots what a game engine is to games.

## Dataflow Robotics with `dora-rs` {#dora-rs}

[`dora-rs`][dora~repo]↗{{hi:dora-rs}} (DORA, Dataflow-Oriented Robotic Architecture) provides middleware for robotics and AI applications. Its applications are described as directed dataflow graphs whose nodes exchange data through the runtime.

## Frameworks {#frameworks .skip}

`Peng`{{hi:Peng}} - A minimal quadrotor autonomy framework

## gRPC {#grpc .skip}

gRPC A high performance, open source universal RPC framework.

[`CleanIt`][cleanit~repo]↗{{hi:CleanIt}} - Open-source Autonomy Software in Rust-lang with [gRPC][p~grpc] for the Roomba series robot vacuum cleaners.

## Related Topics {#related-topics .skip}

- Robotics middleware and protocols.
- Control systems and kinematics.
- Dataflow architectures.
- [ROS](./robot_operating_systems.md) and native Rust robotics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [useful_robotics_tools_and_libs: locate libs, organize, write](https://github.com/john-cd/rust_howto/issues/479)
</div>
