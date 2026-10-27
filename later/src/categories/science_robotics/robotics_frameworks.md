<!-- TODO Reviewed ??-??-2026 -->

# Robotics Frameworks

{{#include robotics_frameworks.incl.md}}

Rust robotics libraries cover several layers: communication, runtime orchestration, robot kinematics, and numerical computation. The table compares their roles and ROS 2 connections. Release information was checked on October 27, 2026; release cadence does not by itself guarantee stability.

| Project                     | Purpose                                                         | ROS 2 interoperability                                                              | Maturity signal                                                                                                                 |
| --------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| [Zenoh][c~zenoh~repo]       | Pub/sub, query, storage, and distributed compute.               | DDS bridge through Zenoh ([bridge][zenoh~ros2~bridge]).                             | v1.10.1 (September 2026); example disabled because `lz4_flex 0.10` is affected ([releases][zenoh~releases]).                    |
| [Dora][dora~repo]           | Distributed dataflow runtime for robotics and AI.               | ROS 2 topics, services, and actions over DDS or Zenoh ([bridge][dora~ros2~bridge]). | v1.1.0 (October 2026; [releases][dora~releases]).                                                                               |
| [Copper][copper~repo]       | Deterministic robot runtime with static task graphs and replay. | Separate [`cu-ros2-bridge`][copper~ros2~bridge]; check release compatibility.       | v1.3.1 (October 2026; [releases][copper~releases]).                                                                             |
| [OpenRR][c~openrr~repo]     | Robot abstractions, clients, planners, and tools.               | ROS 2 support is experimental, per upstream.                                        | v0.1.0; upstream says it is heavily under development; example disabled pending dependency fixes ([releases][openrr~releases]). |
| [`k`][c~k~repo]             | URDF loading, forward kinematics, and inverse kinematics.       | No built-in transport.                                                              | v0.32.0 (September 2026; [releases][k~releases]).                                                                               |
| [nalgebra][c~nalgebra~repo] | Linear algebra and geometry; used by `k`.                       | No built-in transport.                                                              | v0.35.0 (May 2026; [changelog][nalgebra~releases]).                                                                             |
| [faer][faer~repo]           | Dense linear algebra and matrix decompositions.                 | No built-in transport.                                                              | v0.24.4 (June 2026; [crate documentation][faer~docs]).                                                                          |
