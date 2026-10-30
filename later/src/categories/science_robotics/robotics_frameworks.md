<!-- TODO Reviewed ??-??-2026 -->

# Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

Rust robotics libraries cover several layers: communication, runtime orchestration, robot kinematics, and numerical computation. The table compares their roles and ROS 2 connections. Release information was checked on October 27, 2026; release cadence does not by itself guarantee stability.

| Project                     | Purpose                                                         | ROS 2 interoperability                                                              | Maturity signal                                                                                                                 |
| --------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| [Zenoh][c~zenoh~repo]       | Pub/sub, query, storage, and distributed compute.               | DDS bridge through Zenoh ([bridge][zenoh~ros2~bridge]).                             | v1.10.2; the local example enables only the TCP transport and documents the `lz4_flex 0.10` advisory exception ([releases][zenoh~releases]).                    |
| [Dora][dora~repo]           | Distributed dataflow runtime for robotics and AI.               | ROS 2 topics, services, and actions over DDS or Zenoh ([bridge][dora~ros2~bridge]). | v1.1.0 (October 2026; [releases][dora~releases]).                                                                               |
| [Copper][copper~repo]       | Deterministic robot runtime with static task graphs and replay. | Separate [`cu-ros2-bridge`][copper~ros2~bridge]; check release compatibility.       | v1.3.1 (October 2026; [releases][copper~releases]).                                                                             |
| [OpenRR][c~openrr~repo]     | Robot abstractions, clients, planners, and tools.               | ROS 2 support is experimental, per upstream.                                        | v0.1.0; upstream says it is heavily under development; example disabled pending dependency fixes ([releases][openrr~releases]). |
| [`k`][c~k~repo]             | URDF loading, forward kinematics, and inverse kinematics.       | No built-in transport.                                                              | v0.32.0 (September 2026; [releases][k~releases]).                                                                               |
| [nalgebra][c~nalgebra~repo] | Linear algebra and geometry; used by `k`.                       | No built-in transport.                                                              | v0.35.0 (May 2026; [changelog][nalgebra~releases]).                                                                             |
| [faer][faer~repo]           | Dense linear algebra and matrix decompositions.                 | No built-in transport.                                                              | v0.24.4 (June 2026; [crate documentation][faer~docs]).                                                                          |

## Publish and Subscribe with Zenoh {#zenoh}

[![zenoh][c~zenoh~docs~badge]][c~zenoh~docs]{{hi:zenoh}}

[`zenoh`][c~zenoh~docs]↗{{hi:zenoh}} moves telemetry and commands between robot components with publishers, subscribers, and queries addressed by key expressions. The example opens one session on this machine (no multicast scouting; a listener on the IPv4 loopback address only), publishes three values, and receives them. Enable it with `--features zenoh`; it uses the TCP transport only. Zenoh's transport crate depends on `lz4_flex` 0.10, which has a decompression advisory (RUSTSEC-2026-0041); the workspace enables the advisory's `safe-decode` mitigation and records the exception in `deny.toml`. Do not accept compressed data from untrusted peers until a Zenoh release updates the dependency.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/robotics_frameworks/zenoh.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
