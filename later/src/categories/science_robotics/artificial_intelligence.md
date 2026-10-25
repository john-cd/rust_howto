<!-- TODO Reviewed ??-??-2026 -->

# Artificial Intelligence for Robotics

{{#include artificial_intelligence.incl.md}}

Robotics AI is still maturing in Rust, but Rust's safety and performance are a good match for robotics AI workloads: low latency, no garbage-collector pauses, and memory safety on the robot. This page covers planning a path, making decisions with behavior trees, solving embedded optimization problems and wiring components into dataflow pipelines. To run a trained model on a robot, see [Deep Learning][p~deep-learning] (`candle`, `tract`, `ort`); general machine learning is in [Classical Machine Learning][p~classical-machine-learning].

## Plan a Path with A* and `pathfinding` {#astar}

[![pathfinding][c~pathfinding~docs~badge]][c~pathfinding~docs]{{hi:pathfinding}}
[![pathfinding~crates.io][c~pathfinding~crates.io~badge]][c~pathfinding~crates.io]
[![pathfinding~repo][c~pathfinding~repo~badge]][c~pathfinding~repo]
[![pathfinding~lib.rs][c~pathfinding~lib.rs~badge]][c~pathfinding~lib.rs]

[`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} implements A*, Dijkstra and other graph algorithms. A mobile robot typically turns its map into an occupancy grid; A* then finds the cheapest path using a heuristic that never overestimates the remaining cost. The example plans a path around walls on a small grid.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/astar.rs:example}}
```

## Create a Behavior Tree with `bonsai-bt` {#bonsai-bt}

[![bonsai-bt][c~bonsai-bt~docs~badge]][c~bonsai-bt~docs]{{hi:bonsai-bt}}
[![bonsai-bt~crates.io][c~bonsai-bt~crates.io~badge]][c~bonsai-bt~crates.io]
[![bonsai-bt~repo][c~bonsai-bt~repo~badge]][c~bonsai-bt~repo]
[![bonsai-bt~lib.rs][c~bonsai-bt~lib.rs~badge]][c~bonsai-bt~lib.rs]

[`bonsai-bt`][c~bonsai-bt~docs]↗{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][behavior-tree~wikipedia]↗ (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs:example}}
```

## Embedded Optimization with `optimization-engine` {#optimization-engine}

[`optimization-engine`][optimization-engine~repo]↗{{hi:optimization-engine}} (OpEn) is a pure Rust framework for fast embedded nonconvex optimization. It is used for model predictive control (MPC) and trajectory planning on microcontrollers and on-board computers, and it generates solvers that can be called from Rust, C or ROS. See [Embedded][p~embedded].

## Dataflow Robotics with `dora-rs` {#dora-rs}

[![dora-rs][c~dora-rs~docs~badge]][c~dora-rs~docs] [![dora-rs~crates.io][c~dora-rs~crates.io~badge]][c~dora-rs~crates.io] [![dora-rs~repo][c~dora-rs~repo~badge]][c~dora-rs~repo] [![dora-rs~lib.rs][c~dora-rs~lib.rs~badge]][c~dora-rs~lib.rs]{{hi:dora-rs}}

[`dora-rs`][c~dora-rs~docs]↗{{hi:dora-rs}} (Dataflow Oriented Robotic Architecture) is a framework for AI robotics applications: nodes, written in Rust or Python, exchange data through a declarative dataflow graph. Its Rust ecosystem is small, so check its maturity before you adopt it.

## Related Topics {#related-topics .skip}

- [[computer-vision | Computer Vision]].
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
