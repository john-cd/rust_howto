# Artificial Intelligence for Robotics

{{#include artificial_intelligence.incl.md}}

Behavior trees, decision-making, machine learning.

Consider [`bonsai-bt`][c~bonsai-bt~docs]↗{{hi:bonsai-bt}} (Behavior Tree implementation) and `optimization-engine`, a pure Rust framework for embedded nonconvex optimization.

## Create a Behavior Tree with `bonsai-bt` {#bonsai-bt}

[![bonsai-bt][c~bonsai-bt~docs~badge]][c~bonsai-bt~docs]{{hi:bonsai-bt}}
[![bonsai-bt~crates.io][c~bonsai-bt~crates.io~badge]][c~bonsai-bt~crates.io]
[![bonsai-bt~github][c~bonsai-bt~github~badge]][c~bonsai-bt~github]
[![bonsai-bt~lib.rs][c~bonsai-bt~lib.rs~badge]][c~bonsai-bt~lib.rs]

[`bonsai-bt`][c~bonsai-bt~docs]↗{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][behavior-tree~wikipedia]↗ (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs:example}}
```

## Related Topics {#related-topics}

- [[computer-vision | Computer Vision]].
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1209)

[behavior-tree (Wikipedia)][wikipedia~behavior-tree].

Rust Crates:

- [`dora-rs`][c~dora-rs~docs]↗{{hi:dora-rs}}: Robotics framework for AI applications. Low usage.
- `optimization-engine`: [Embedded][p~embedded] optimization for robotics.

[![dora-rs][c~dora-rs~docs~badge]][c~dora-rs~docs] [![dora-rs~crates.io][c~dora-rs~crates.io~badge]][c~dora-rs~crates.io] [![dora-rs~github][c~dora-rs~github~badge]][c~dora-rs~github] [![dora-rs~lib.rs][c~dora-rs~lib.rs~badge]][c~dora-rs~lib.rs]{{hi:dora-rs}}

Dataflow Oriented Robotic Architecture: TODO

</div>
