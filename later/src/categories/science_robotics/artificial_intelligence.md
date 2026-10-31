<!-- TODO Reviewed ??-??-2026 -->

# Artificial Intelligence for Robotics

{{#include artificial_intelligence.incl.md}}

Robotics AI is still maturing in Rust, but Rust's safety and performance are a good match for robotics AI workloads: low latency, no garbage-collector pauses, and memory safety on the robot. This page covers planning a path, making decisions with behavior trees, solving embedded optimization problems and wiring components into dataflow pipelines. To run a trained model on a robot, see [Deep Learning][p~deep-learning] (`candle`, `tract`, `ort`); general machine learning is in [Classical Machine Learning][p~classical-machine-learning].

## Plan a Path with A* and `pathfinding` {#astar}

{{#badges pathfinding}}

[`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} implements A*, Dijkstra and other graph algorithms. A mobile robot typically turns its map into an occupancy grid; A* then finds the cheapest path using a heuristic that never overestimates the remaining cost. The example plans a path around walls on a small grid.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/astar.rs:example}}
```

## Create a Behavior Tree with `bonsai-bt` {#bonsai-bt}

{{#badges bonsai-bt}}

[`bonsai-bt`][c~bonsai-bt~docs]↗{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][wikipedia~behavior-tree]↗ (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs:example}}
```

## Embedded Optimization with `optimization-engine` {#optimization-engine}

[`optimization-engine`][optimization-engine~repo]↗{{hi:optimization-engine}} (OpEn) is a pure Rust framework for fast embedded nonconvex optimization. It is used for model predictive control (MPC) and trajectory planning on microcontrollers and on-board computers, and it generates solvers that can be called from Rust, C or ROS. See [Embedded][p~embedded].

## Connect Robotics Components with Dora {#dora}

[Dora][dora~website]↗ describes robotics and AI pipelines as dataflows: nodes exchange inputs and outputs through a configured graph. Use the framework when a perception, planning, and control pipeline needs separately managed components; consult its documentation for runtime setup and integration details.

## Related Topics {#related-topics .skip}

- [[computer-vision | Computer Vision]].
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
